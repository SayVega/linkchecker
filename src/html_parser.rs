use crate::model::LinkError;
use regex::Regex;

pub fn extract_title(html: &str) -> Result<String, LinkError> {
    let re = Regex::new(r"(?is)<title>(.*?)</title>").unwrap();
    match re.captures(html) {
        Some(caps) => Ok(caps.get(1).unwrap().as_str().to_string()),
        None => Err(LinkError::MissingTitle),
    }
}
