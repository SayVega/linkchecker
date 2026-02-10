use linkchecker::processor::process_links;
use linkchecker::text_parser::parse_file;
use linkchecker::writer::write_results;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn linkchecker_produces_expected_output() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.md");
    let output_path = dir.path().join("output.md");
    fs::write(
        &input_path,
        r#"
        [Example](https://example.com)
        [Broken](http://127.0.0.1:9)
        "#,
    )
    .unwrap();
    let links = parse_file(input_path.to_str().unwrap()).unwrap();
    let results = process_links(links).await;
    write_results(output_path.to_str().unwrap(), &results).unwrap();
    let output = fs::read_to_string(output_path).unwrap();
    assert!(output.contains("[Example Domain]("));
    assert!(output.contains("NETWORK_ERROR"));
}
