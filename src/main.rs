mod model;
mod text_parser;
mod processor;
mod html_parser;

use processor::process_links;
use text_parser::parse_file;
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
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        }
    }
}
