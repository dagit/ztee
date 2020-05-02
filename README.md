# ztee

Works similarly to `tee` but the file output is `gzip` compressed.

This is a very simple program and probably lacks features of a real `tee`
implementation, but should still be useful in many cases.

## Installation

1. From crates.io:

```
cargo install ztee
```

## Usage

```
$ ztee --help
ztee is like tee but with gzipped file output 0.1.0
Jason Dagit <dagitj@gmail.com>
Duplicates stdin to stdout and also a compressed file.

USAGE:
    ztee <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    Write compressed stream to FILE
```
