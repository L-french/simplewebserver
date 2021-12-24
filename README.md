# simplewebserver

A convenient, standalone web server in the style of Python's `SimpleHTTPServer`. 

*Warning*: Like its Python cousin, simplewebserver is intended for development and testing. It should not be used in production environments.

## Design Goals
* **Simple**: Easy to use CLI, with good defaults and reasonable security
* **Capable**: Multithreaded service and I/O powered by Tokio
* **Tiny**: Minimal binary size, while still using mature libraries and upholding other goals

## Installation

`cargo install simplewebserver` TODO

## Usage

```
USAGE:
    simplewebserver [FLAGS] [OPTIONS] <FILE>...

FLAGS:
    -D, --dry-run      Print files which would be served and exit
    -h, --help         Prints help information
    -r, --recursive    Serve directories recursively
    -V, --version      Prints version information

OPTIONS:
    -p, --port <port>    Bind to a port [default: 7878]

ARGS:
    <FILE>...    The file(s) to serve
```
