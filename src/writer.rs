use std::fs::File;
use std::io::{BufWriter, Write};

use crate::model::{LinkError, LinkResult};

pub fn write_results(path: &str, results: &[LinkResult]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for result in results {
        match &result.result {
            Ok(title) => {
                writeln!(writer, "[{}]({})", title, result.link.url)?;
            }
            Err(err) => {
                writeln!(
                    writer,
                    "[{} from {}]({})",
                    format_error(err),
                    result.link.text,
                    result.link.url
                )?;
            }
        }
    }
    return Ok(());
}

fn format_error(err: &LinkError) -> &'static str {
    match err {
        LinkError::Timeout => "TIMEOUT",
        LinkError::Network => "NETWORK_ERROR",
        LinkError::InvalidStatus(code) => match *code {
            404 => "NOT_FOUND",
            500..=599 => "SERVER_ERROR",
            _ => "HTTP_ERROR",
        },
        LinkError::InvalidHtml => "INVALID_HTML",
        LinkError::MissingTitle => "MISSING_TITLE",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Link, LinkError, LinkResult};
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn writes_ok_result() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        let results = LinkResult {
            link: Link {
                text: "Example".into(),
                url: "https://example.com".into(),
            },
            result: Ok("Example Domain".into()),
        };
        write_results(path.to_str().unwrap(), &[results]).unwrap();
        let contents = fs::read_to_string(path).unwrap();
        assert_eq!(contents, "[Example Domain](https://example.com)\n");
    }
    #[test]
    fn write_error_result_formats_correctly() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        let result = LinkResult {
            link: Link {
                url: "https://example.com".to_string(),
                text: "example link".to_string(),
            },
            result: Err(LinkError::MissingTitle),
        };
        write_results(path.to_str().unwrap(), &[result]).unwrap();
        let contents = fs::read_to_string(path).unwrap();
        assert_eq!(
            contents,
            "[MISSING_TITLE from example link](https://example.com)\n"
        );
    }
}
