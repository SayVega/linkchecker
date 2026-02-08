#[derive(Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
}
#[derive(Debug)]
pub struct LinkResult {
    pub link: Link,
    pub result: Result<String, LinkError>,
}
#[derive(Debug)]
pub enum LinkError {
    Network,
    Timeout,
    InvalidStatus(u16),
    InvalidHtml,
    MissingTitle,
}
