# simplewebserver

A convenient, standalone web server in the style of Python's `SimpleHTTPServer`. 

*Warning*: Like its Python cousin, simplewebserver is intended for development and testing; it should not be used in production environments.

## Design Goals
* **Simple**: Easy to use CLI, with good defaults and reasonable security
* **Capable**: Multithreaded service and I/O powered by Tokio
* **Tiny**: Minimal binary size, while still using mature libraries and upholding other goals

## Installation

`cargo install simplewebserver` TODO

## Usage

```
USAGE:
    simplewebserver <FILE>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>...    The file(s) to serve
```