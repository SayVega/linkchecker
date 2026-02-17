use crate::html_parser::extract_title;
use crate::model::{Link, LinkError, LinkResult};
use futures::stream::{self, StreamExt};
use std::time::Duration;

pub async fn process_link(client: &reqwest::Client, link: Link) -> LinkResult {
    let response = match client.get(&link.url).send().await {
        Ok(resp) => resp,
        Err(err) => {
            let error = if err.is_timeout() {
                LinkError::Timeout
            } else {
                LinkError::Network
            };
            return LinkResult {
                link,
                result: Err(error),
            };
        }
    };
    let status = response.status();
    if !status.is_success() {
        return LinkResult {
            link,
            result: Err(LinkError::InvalidStatus(status.as_u16())),
        };
    };
    let body = match response.text().await {
        Ok(text) => text,
        Err(_) => {
            return LinkResult {
                link,
                result: Err(LinkError::InvalidHtml),
            };
        }
    };
    return match extract_title(&body) {
        Ok(title) => LinkResult {
            link,
            result: Ok(title),
        },
        Err(err) => LinkResult {
            link,
            result: Err(err),
        },
    };
}

pub async fn process_links(links: Vec<Link>) -> Vec<LinkResult> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    stream::iter(links)
        .map(|link| process_link(&client, link))
        .buffer_unordered(32)
        .collect()
        .await
}

#[cfg(test)]
mod tests {
    mod getting_request {
        use crate::model::{Link, LinkError};
        use crate::processor::process_link;
        use httpmock::prelude::*;
        use reqwest::Client;
        use std::time::Duration;

        fn test_client(timeout_secs: u64) -> Client {
            Client::builder()
                .timeout(Duration::from_secs(timeout_secs))
                .build()
                .unwrap()
        }
        #[tokio::test]
        async fn process_link_success() {
            let server = MockServer::start();
            server.mock(|when, then| {
                when.method(GET).path("/ok");
                then.status(200)
                    .header("Content-Type", "text/html")
                    .body("<html><head><title>Hello</title></head></html>");
            });
            let link = Link {
                text: "test".into(),
                url: format!("{}/ok", server.url("")),
            };
            let result = process_link(&test_client(5), link).await;
            assert!(result.result.is_ok());
        }
        #[tokio::test]
        async fn process_link_network_error() {
            let link = Link {
                text: "test".into(),
                url: "http://127.0.0.1:9".into(),
            };
            let result = process_link(&test_client(5), link).await;
            assert!(matches!(result.result, Err(LinkError::Network)));
        }
        #[tokio::test]
        async fn process_link_http_error() {
            let server = MockServer::start();
            server.mock(|when, then| {
                when.method(GET).path("/404");
                then.status(404);
            });
            let link = Link {
                text: "test".into(),
                url: format!("{}/404", server.url("")),
            };
            let result = process_link(&test_client(5), link).await;
            assert!(matches!(result.result, Err(LinkError::InvalidStatus(404))));
        }
        #[tokio::test]
        async fn process_link_missing_title_tag() {
            let server = MockServer::start();
            server.mock(|when, then| {
                when.method(GET).path("/no-title");
                then.status(200)
                    .header("Content-Type", "text/html")
                    .body("<html><head></head><body>No title here</body></html>");
            });
            let link = Link {
                text: "missing".into(),
                url: format!("{}/no-title", server.url("")),
            };
            let result = process_link(&test_client(5), link).await;
            assert!(matches!(result.result, Err(LinkError::MissingTitle)));
        }
        #[tokio::test]
        async fn process_link_empty_title_is_valid() {
            use httpmock::prelude::*;
            let server = MockServer::start();
            server.mock(|when, then| {
                when.method(GET).path("/empty-title");
                then.status(200)
                    .header("Content-Type", "text/html")
                    .body("<html><head><title></title></head></html>");
            });
            let link = Link {
                text: "empty".into(),
                url: format!("{}/empty-title", server.url("")),
            };
            let result = process_link(&test_client(5), link).await;
            match result.result {
                Ok(title) => assert_eq!(title, ""),
                Err(e) => panic!("unexpected error: {:?}", e),
            }
        }
        #[tokio::test]
        async fn process_link_timeout() {
            use httpmock::prelude::*;
            use std::time::Duration;
            let server = MockServer::start();
            server.mock(|when, then| {
                when.method(GET).path("/slow");
                then.delay(Duration::from_secs(5))
                    .status(200)
                    .body("<html><head><title>Slow</title></head></html>");
            });
            let link = Link {
                text: "slow".into(),
                url: format!("{}/slow", server.url("")),
            };
            let result = process_link(&test_client(1), link).await;
            assert!(matches!(result.result, Err(LinkError::Timeout)));
        }
    }
    mod concurrency {
        #[tokio::test]
        async fn process_links_order_is_not_preserved() {
            use crate::model::Link;
            use crate::processor::process_links;
            use httpmock::prelude::*;
            use std::time::Duration;
            let server = MockServer::start();
            let delays = [300, 100, 200];
            for (i, delay) in delays.iter().enumerate() {
                server.mock(move |when, then| {
                    when.method(GET).path(format!("/{}", i));
                    then.status(200)
                        .delay(Duration::from_millis(*delay))
                        .body(format!("<html><head><title>{}</title></head></html>", i));
                });
            }
            let links: Vec<Link> = (0..3)
                .map(|i| Link {
                    text: format!("link{}", i),
                    url: format!("{}/{}", server.url(""), i),
                })
                .collect();
            let results = process_links(links.clone()).await;
            assert_eq!(results.len(), links.len());
            let input_urls: Vec<_> = links.iter().map(|l| &l.url).collect();
            let output_urls: Vec<_> = results.iter().map(|r| &r.link.url).collect();
            assert_ne!(input_urls, output_urls);
        }
        #[tokio::test]
        async fn process_links_runs_in_parallel() {
            use crate::model::Link;
            use crate::processor::process_links;
            use httpmock::prelude::*;
            use std::time::{Duration, Instant};
            let server = MockServer::start();
            let delay = Duration::from_millis(200);
            let n = 8;
            for i in 0..n {
                server.mock(move |when, then| {
                    when.method(GET).path(format!("/{}", i));
                    then.status(200)
                        .delay(delay)
                        .body("<html><head><title>ok</title></head></html>");
                });
            }
            let links: Vec<Link> = (0..n)
                .map(|i| Link {
                    text: format!("link{}", i),
                    url: format!("{}/{}", server.url(""), i),
                })
                .collect();
            let start = Instant::now();
            let _results = process_links(links).await;
            let elapsed = start.elapsed();
            assert!(
                elapsed < delay * 3,
                "process_links took {:?}, expected parallel execution",
                elapsed
            );
        }
    }
}
