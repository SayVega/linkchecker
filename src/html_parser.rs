use crate::model::LinkError;
use regex::Regex;

pub fn extract_title(html: &str) -> Result<String, LinkError> {
    let re = Regex::new(r"(?is)<title[^>]*>(.*?)</title>").unwrap();
    match re.captures(html) {
        Some(caps) => Ok(caps
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")),
        None => Err(LinkError::MissingTitle),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_simple_title() {
        let html = "<html><head><title>Example Title</title></head><body></body></html>";
        assert_eq!(extract_title(html).unwrap(), "Example Title");
    }
    #[test]
    fn extract_title_normalizes_whitespace() {
        let html = "<title>\n\n   Whitespace   \n\r  title \n</title>";
        assert_eq!(extract_title(html).unwrap(), "Whitespace title");
    }
    #[test]
    fn extract_empty_title_is_valid() {
        let html = "<title></title>";
        assert_eq!(extract_title(html).unwrap(), "");
    }
    #[test]
    fn title_is_case_insensitive() {
        let html = "<TITLE>Upper</TITLE>";
        assert_eq!(extract_title(html).unwrap(), "Upper");
    }
    #[test]
    fn first_title_is_used() {
        let html = "<title>One</title><title>Two</title>";
        assert_eq!(extract_title(html).unwrap(), "One");
    }
}
