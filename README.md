# Corpus Preprocessor

[![Build binary](https://github.com/dosjorge/corpus-preproc/actions/workflows/release.yml/badge.svg)](https://github.com/dosjorge/corpus-preproc/actions/workflows/release.yml)
![Crates.io](https://img.shields.io/crates/v/corpus-preproc)
[![DOI](https://zenodo.org/badge/456014248.svg)](https://zenodo.org/badge/latestdoi/456014248)

CLI and HTTP API to preprocess corpora for word embeddings and possibly other NLP tasks. The main goal is to convert
many HTML or plain text files into a single normalized plain text corpus.


## Features
- Parallel processing of files in a directory (CLI only)
- NKFC and whitespace normalization
- Removal of modifiers and marks
- Lower-case folding
- Trimming of punctuation around words
- Replace words with `<unk>` placeholder if they meet any of the following criteria:
  - Word has an at sign `@`
  - Word lacks alphabetic characters
  - Word has two punctuation chars in a row, such as `http://`
- HTML code is parsed and CSS selectors can be used to:
  - Remove undesired elements
  - Insert newlines after paragraphs and line breaks
  - Extract the main content of an HTML document
- Text is automatically converted to UTF-8 if the original encoding is in the
  [Encoding Standard](https://encoding.spec.whatwg.org/#names-and-labels).

## Usage
### Command Line Interface (CLI)
```console
# Install
$ cargo install corpus-preproc
# Run CLI help
$ corpus-preproc clean -h
Preprocess a file or directory

USAGE:
    corpus-preproc clean [OPTIONS] <INPUT> <OUTPUT>

ARGS:
    <INPUT>     
    <OUTPUT>    

OPTIONS:
    -c
            Clean HTML tags

        --content-selector <CONTENT_SELECTOR>
            CSS selector for main content

        --delete-selector <DELETE_SELECTOR>
            CSS selector for tag removal [default: "script, style, pre, svg, math, noscript, ref,
            table, tr, td, ol, ul, li, time, [aria-hidden], img, figure"]

    -h, --help
            Print help information

    -l
            Perform case-folding

    -m
            Keep modifiers and marks on normalization

    -n
            Perform NFKC and whitespace normalization

        --nl-append-selector <NL_APPEND_SELECTOR>
            CSS selector to append newline [default: "div, p, hr, br, h1, h2, h3, h4, h5, h6"]

    -p
            Trim punctuation surrounding words

    -t <THREADS>
            Number of threads to use [default: 4]
```
### HTTP API

#### Startup
```console
$ corpus-preproc serve 127.0.0.1:8000
```
#### Python Example
The [`requests`](https://docs.python-requests.org/en/latest/user/install/) Python library needs to be installed.
```python
import requests
import json

DEFAULT_CONFIG = {
  "htmlClean": {
    "enabled": True,
    "contentSelector": None,
    "deleteSelector": "script, style, pre, svg, math, noscript, ref, table, tr, td, ol, ul, li, time, [aria-hidden], img, figure",
    "nlAppendSelector": "div, p, hr, br, h1, h2, h3, h4, h5, h6",
  },
  "charNormalization": {
    "enabled": True,
    "keepModifiersAndMarks": False,
    "lowercase": True,
  },
  "wordNormalization": {
    "enabled": True,
    "replacePii": True,
  }
}

def clean_text(text):
    files = {
        'config': (None, json.dumps(DEFAULT_CONFIG), 'application/json'), # optional
        'data': (None, text, 'text/plain'),
    }
    response = requests.post('http://127.0.0.1:3000/preproc', files=files)
    return response.text
clean = clean_text("<b>HELLo, WORLD!!!").rstrip()
assert (clean == "hello world"), "OK"
```