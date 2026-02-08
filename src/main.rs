mod html_parser;
mod model;
mod processor;
mod text_parser;

use processor::process_links;
use std::env;
use text_parser::parse_file;

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
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        }
    }
}
