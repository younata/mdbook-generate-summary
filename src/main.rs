use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use glob::glob;

use crate::summary_entry::SummaryEntry;
use itertools::Itertools;

mod summary_entry;

fn find_content(path: &std::path::PathBuf, base_path: &std::path::PathBuf) -> Option<SummaryEntry> {
    File::open(path).map(|f| {
        BufReader::new(f).lines()
            .filter_map(|l| l.ok())
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with("# ") {
                    Some(trimmed.trim_start_matches("# ").to_string())
                } else {
                    None
                }
            })
            .next().map(|title| {
            SummaryEntry {
                path: relative_path(path, base_path),
                title,
            }
        })
    }).unwrap_or(None)
}

fn relative_path(path: &std::path::PathBuf, base_path: &std::path::PathBuf) -> std::path::PathBuf {
    path.strip_prefix(base_path)
        .expect("Given a base path that is not actually a base path for current directory")
        .to_path_buf()
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str), default_value="src/")]
    base_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let base_path = args.base_path;
    let glob_string = String::from(base_path.to_str().expect("given path should be a string")) + "/**/*.md";
    let entries = glob(&glob_string)
        .expect("Failed to read glob pattern")
        .filter_map(|e| e.ok())
        .filter_map(|e| find_content(&e, &base_path));

    let entry_lines = entries.into_iter().sorted().map(|e| e.summary_line());

    let mut summary = File::create(base_path.join("SUMMARY.md"))
        .expect("Failed to create SUMMARY.md");

    for line in entry_lines {
        writeln!(summary, "{}", line).expect("Unable to write to file");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn find_content_finds_a_title() {
        let mut file = NamedTempFile::new().expect("should have created a tempfile");
        writeln!(file, "\n\n# A File").expect("should have written to file");

        let entry = find_content(&file.path().to_path_buf(), &file.path().parent().unwrap().to_path_buf());

        assert_eq!(entry.is_none(), false);

        if let Some(ref entry) = entry {
            assert_eq!(entry.title, "A File".to_string());
        }
    }

    #[test]
    fn find_content_returns_no_title_if_none_found() {
        let mut file = NamedTempFile::new().expect("should have created a tempfile");
        writeln!(file, "\n\n").expect("should have written to file");

        let entry = find_content(&file.path().to_path_buf(), &file.path().parent().unwrap().to_path_buf());

        assert_eq!(entry.is_none(), true);
    }
}
