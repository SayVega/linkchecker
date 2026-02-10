use linkchecker::processor::process_links;
use linkchecker::text_parser::parse_file;
use linkchecker::writer::write_results;
use std::env;

#[tokio::main]
async fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();
    let path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("Usage: {} <input_file>", program);
            std::process::exit(1);
        }
    };
    match parse_file(&path) {
        Ok(links) => {
            let results = process_links(links).await;
            if let Err(e) = write_results("output.md", &results) {
                eprintln!("Error writing output: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        }
    }
}
