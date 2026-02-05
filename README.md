# linkchecker

Linkchecker is a Rust-based tool that analyzes a text file containing URLs in Markdown format, validates each link, and generates a new enriched Markdown file.

## Details

 The new enriched Markdown file will be formatted like this:

- [ EXTRACTED_TITLE ] ( URL ) if the request was succesful.
- [ HUMAN READABLE ERROR CODE ] ( URL )

## Documentation

Detailed documentation is available [`here`](docs/README.md).