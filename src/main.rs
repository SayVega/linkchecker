mod model;
mod parser;
use parser::parse_file;
use std::env;
fn main() {
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
        Ok(links) => {}
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        }
    }
}
