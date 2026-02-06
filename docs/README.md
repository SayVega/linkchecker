# Linkchecker

Linkchecker is a command-line tool written in Rust that analyzes a text file and processes links written in inline Markdown format.

## Parsing

The program parses a text file line by line and extracts inline Markdown links.

It scans the input file (typically a `.md` file) and identifies links written exactly in the following form:

`[text](url)`

Links are extracted using a regular expression that captures the link text and the link target.  

No URL validation is performed at this stage; extracted URLs are processed independently in later phases.

## Obtaining data from the URLs

Once the parsing part obtains the `Links`


## Notes
global request timeout = 5 seconds
process_links accepts empty titles