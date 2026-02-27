# Linkchecker

Linkchecker is a command-line tool written in Rust that analyzes a text file and processes links written in inline Markdown format.

## Prerequisites
* Rust (latest stable)
* OpenSSL headers (Linux only: `libssl-dev` or `openssl-devel`)
* `make` (optional)


## Installation

Clone the repository and navigate to the directory
```
git clone https://github.com/SayVega/linkchecker.git
cd linkchecker
```
You may optionally install it system-wide:
```
cargo install --path .
```
## Usage
### Using make
Compile the binary
```
make build
```
Run the link checker using a `.txt`/`.md` file as argument:
```
make run ARGS="example_pages.md"
```

### Using cargo (Manual)
For better performance always use `--release` flag

Build:
```
cargo build --release
```
Run:
```
cargo run --release -- available_pages.md
```

Note: Output order is not guaranteed due to concurrent processing.

### Testing
The suite includes integration tests and unit tests.
```
make test
```

## Processing pipeline

### Parsing

The program parses a text file line by line and extracts inline Markdown links.

It scans the input file (typically a `.md` file) and identifies links written **exactly** in the following form:

`[text](url)`

Links are extracted using a regular expression that captures the link text and the link target.  

No URL validation is performed at this stage; extracted URLs are processed independently in later phases.

### Concurrent link processing

All extracted links are processed concurrently, up to **32 URLs at a time**

For each URL, the program:

1) Sends a HTTP GET request.
2) Checks the HTTP status code.
3) If successful, reads the response body.
4) Parses the HTML and extracts the `<title>` value.

The global request timeout is **5 seconds** per URL.

Empty title tags (`<title></title>`) are considered valid and produce an empty title.

### Output generation

The results are written to a new Markdown file (`output.md`).

Sucessful request are written as: `[title](url)`

Failed request are written as: `[error_code from original_text](url)`

Error codes are **human-readable** and cover:
| Error Code | Description |
| :--- | :--- |
| **TIMEOUT** | The request exceeded the 5-second limit. |
| **NETWORK_ERROR** | Connection issues, including DNS resolution failures and refused connections. |
| **NOT_FOUND** | HTTP 404 status code. |
| **SERVER_ERROR** | HTTP status codes in the 500-599 range. |
| **HTTP_ERROR** | Any other non-successful HTTP status code. |
| **INVALID_HTML** | Response body could not be parsed as HTML. |
| **MISSING_TITLE** | The HTML was parsed but no title tag was found. |
| **EMPTY_TITLE** | The HTML was parsed and the title is empty |