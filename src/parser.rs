use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use crate::model::Link;

pub(crate) fn parse_file(path: &str) -> std::io::Result<Vec<Link>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap(); // Matches inline Markdown links: [text](target)
    let mut links = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        for capture in link_regex.captures_iter(&line) {
            let text = capture[1].to_string();
            let url = capture[2].to_string();
            links.push(Link { text, url });
        }
    }
    return Ok(links);
}

#[cfg(test)]
mod tests {
    use super::*;
    mod file_parsing {
        use super::*;
        use std::fs::{self, File};
        use std::io::Write;
        fn setup_file(filename: &str, content: &str) -> String {
            let mut file = File::create(filename).expect("Failed to create test file");
            write!(file, "{}", content).expect("Failed to write to test file");
            return filename.to_string();
        }
        fn teardown_file(filename: &str) {
            let _ = fs::remove_file(filename);
        }
        #[test]
        fn empty_file_returns_empty_vec() {
            let path = setup_file("test_empty.md", "");
            let result = parse_file(&path).unwrap();
            assert!(result.is_empty());
            teardown_file(&path);
        }
        #[test]
        fn simple_link_extraction() {
            let content = "Hello [example](https://example.com)";
            let path = setup_file("test_simple.md", content);
            let result = parse_file(&path).unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].text, "example");
            assert_eq!(result[0].url, "https://example.com");
            teardown_file(&path);
        }

        #[test]
        fn multiple_links_same_line() {
            let content = "See [first](https://first.com) and [second](https://second.com)";
            let path = setup_file("test_multi.md", content);
            let result = parse_file(&path).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].text, "first");
            assert_eq!(result[1].text, "second");
            teardown_file(&path);
        }

        #[test]
        fn regex_ignores_malformed_links() {
            let content = "
                [Valid](http://ok.com)
                [Broken] (http://with-space.com)
                (twisted)[http://twisted.com]
                [noURL]
            ";
            let path = setup_file("test_malformed.md", content);
            let result = parse_file(&path).unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].text, "Valid");
            teardown_file(&path);
        }
    }
}