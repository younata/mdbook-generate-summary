use std::iter::repeat;
use std::cmp::Ordering;
use std::ffi::OsStr;

#[derive(Debug, Eq)]
pub struct SummaryEntry {
    pub path: std::path::PathBuf,
    pub title: String,
}

impl SummaryEntry {
    pub fn summary_line(&self) -> String {
        let indentation = repeat("    ").take(self.indentation_level()).collect::<String>();
        indentation + "- " + &self.link()
    }

    fn indentation_level(&self) -> usize {
        let amount = self.path_string().matches('/').count();
        if amount == 0 {
            return 0
        } else if self.path_string().ends_with("README.md") {
            return amount - 1
        }
        amount
    }

    fn link(&self) -> String {
        format!("[{}]({})", self.title, self.path_string())
    }

    fn path_string(&self) -> &str {
        self.path.to_str().expect("Expected path to convert to string")
    }

    fn html_path(&self) -> std::path::PathBuf {
        if self.path.file_name() == Some(OsStr::new("README.md")) {
            self.path.parent().unwrap_or(std::path::Path::new("")).to_path_buf()
        } else {
            self.path.clone()
        }
    }
}

impl Ord for SummaryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.html_path().cmp(&other.html_path())
    }
}

impl PartialOrd for SummaryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SummaryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.title == other.title
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    use itertools::Itertools;

    #[test]
    fn ordered() {
        let entries = vec![
            SummaryEntry { path: PathBuf::from("README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("foo/bar.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("foo/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("baz/bar.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("baz/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("baz/aoeu.md"), title: "Some Title".to_string() },
        ];

        let sorted_entries = vec![
            SummaryEntry { path: PathBuf::from("README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("baz/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("baz/aoeu.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("baz/bar.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("foo/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("foo/bar.md"), title: "Some Title".to_string() },
        ];

        itertools::assert_equal(entries.into_iter().sorted(), sorted_entries)
    }

    #[test]
    fn ordered_2() {
        let entries = vec![
            SummaryEntry { path: PathBuf::from("food/recipes/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("docker/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ci/concourse.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ios/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("food/recipes/soup.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("meta/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ci/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("food/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ios/popover.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("rust/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("meta/setup.md"), title: "Some Title".to_string() },
        ];

        let sorted_entries = vec![
            SummaryEntry { path: PathBuf::from("README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ci/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ci/concourse.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("docker/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("food/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("food/recipes/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("food/recipes/soup.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ios/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("ios/popover.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("meta/README.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("meta/setup.md"), title: "Some Title".to_string() },
            SummaryEntry { path: PathBuf::from("rust/README.md"), title: "Some Title".to_string() },
        ];

        itertools::assert_equal(entries.into_iter().sorted(), sorted_entries)
    }

    #[test]
    fn summary_line_with_no_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("README.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(
            entry.summary_line(),
            "- [A Title](README.md)"
        );
    }

    #[test]
    fn summary_line_with_1_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(
            entry.summary_line(),
            "    - [A Title](baz/other.md)"
        );
    }

    #[test]
    fn summary_line_with_1_subdir_readme() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/README.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(
            entry.summary_line(),
            "- [A Title](baz/README.md)"
        );
    }

    #[test]
    fn indentation_level_with_base_dir_and_readme() {
        let entry = SummaryEntry {
            path: PathBuf::from("README.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(), 0);
    }

    #[test]
    fn indentation_level_with_base_dir() {
        let entry = SummaryEntry {
            path: PathBuf::from("other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(), 0);
    }

    #[test]
    fn indentation_level_with_1_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(), 1);
    }

    #[test]
    fn indentation_level_with_2_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/qux/other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(), 2);
    }

    #[test]
    fn indentation_level_with_3_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/qux/whatever/other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(), 3);
    }
}