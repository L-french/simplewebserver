# simplewebserver

A convenient, standalone web server in the style of Python's `SimpleHTTPServer`. 

![Example usage of simplewebserver](assets/example-1.gif "Example")

*Warning*: Like its Python cousin, simplewebserver is intended for development and local file sharing. It should not be used in production environments.

## Design Goals
* **Simple**: Easy to use CLI, with good defaults and reasonable security
* **Capable**: Multithreaded service and I/O powered by Tokio
* **Tiny**: Minimal binary size, while still using mature libraries and upholding other goals

## Installation

Linux x64 binaries are available in GitHub Releases. More sophisticated packaging coming soon.

If you have a functional Rust installation, simplewebserver can be installed from crates.io:
```
cargo install simplewebserver
```
Make sure the install location `~/.cargo/bin/` is in your path. Rust installation with rustup does this automatically.

## Usage

```
USAGE:
    simplewebserver [FLAGS] [OPTIONS] <FILE>...

FLAGS:
    -D, --dry-run      Print files which would be served and exit
    -h, --help         Prints help information
    -r, --recursive    Serve directories recursively
    -V, --version      Prints version information
    -v, --verbose      Print additional logging info

OPTIONS:
    -a, --address <address>    Serve on IP address [default: 127.0.0.1]
    -p, --port <port>          Bind to a port [default: 8080]

ARGS:
    <FILE>...    The file(s) to serve
```
