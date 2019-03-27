use std::iter::repeat;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct SummaryEntry {
    pub path: std::path::PathBuf,
    pub title: String,
}

impl SummaryEntry {
    pub fn summary_line(&self) -> String {
        let indentation = repeat("    ").take(self.indentation_level(true)).collect::<String>();
        indentation + "- " + &self.link()
    }

    fn indentation_level(&self, dedent_for_readme: bool) -> usize {
        let amount = self.path_string().matches('/').count();
        if amount == 0 {
            return 0
        } else if dedent_for_readme && self.path_string().ends_with("README.md") {
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
}

impl Ord for SummaryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let indentation_ordering = self.indentation_level(false).cmp(&other.indentation_level(false));
        if indentation_ordering == Ordering::Equal {
            self.path.cmp(&other.path)
        } else {
            indentation_ordering
        }
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
        assert_eq!(entry.indentation_level(true), 0);
    }

    #[test]
    fn indentation_level_with_base_dir() {
        let entry = SummaryEntry {
            path: PathBuf::from("other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(true), 0);
    }

    #[test]
    fn indentation_level_with_1_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(true), 1);
    }

    #[test]
    fn indentation_level_with_2_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/qux/other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(true), 2);
    }

    #[test]
    fn indentation_level_with_3_subdir() {
        let entry = SummaryEntry {
            path: PathBuf::from("baz/qux/whatever/other.md"),
            title: "A Title".to_string(),
        };
        assert_eq!(entry.indentation_level(true), 3);
    }
}