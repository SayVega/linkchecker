# Linkchecker

Linkchecker is a command-line tool written in Rust that analyzes a text file and processes links written in inline Markdown format.

## Instalation

### From source

```
git clone https://github.com/SayVega/linkchecker.git
cd linkchecker
cargo build --release
```

Then you will find the binary at:

`target/release/linkchecker`

You may optionally install it system-wide:

```
cargo install --path .
```

## Usage

```
linkchecker <input_file>
```

Example

```
linkchecker available_pages.md
```

Note: Output order is not guaranteed due to concurrent processing.

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

PD: Error codes are human-readable