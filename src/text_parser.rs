use crate::model::Link;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_file(path: &str) -> std::io::Result<Vec<Link>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    return parse_reader(reader);
}

fn parse_reader<R: BufRead>(reader: R) -> std::io::Result<Vec<Link>> {
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
    use std::io::Cursor;
    #[test]
    fn empty_content_returns_empty_vec() {
        let content = "";
        let cursor = Cursor::new(content);
        let result = parse_reader(cursor).unwrap();
        assert!(result.is_empty());
    }
    #[test]
    fn simple_link_extraction() {
        let content = "Hello [example](https://example.com)";
        let cursor = Cursor::new(content);
        let result = parse_reader(cursor).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "example");
        assert_eq!(result[0].url, "https://example.com");
    }
    #[test]
    fn multiple_links_same_line() {
        let content = "See [first](https://first.com) and [second](https://second.com)";
        let cursor = Cursor::new(content);
        let result = parse_reader(cursor).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].text, "first");
        assert_eq!(result[1].text, "second");
    }
    #[test]
    fn regex_ignores_malformed_links() {
        let content = "
            [Valid](http://ok.com)
            [Broken] (http://with-space.com)
            (twisted)[http://twisted.com]
            [noURL]
        ";
        let cursor = Cursor::new(content);
        let result = parse_reader(cursor).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].text, "Valid");
    }
}
