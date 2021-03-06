# crab

[![crates.io](https://img.shields.io/crates/v/crab.svg)](https://crates.io/crates/crab)

crab is command line web scraping tool written in rust

## features

- supporting css selectors
- showing content of tags or attributes value
- showing DOM structure in tree like form
- support HTTP/POST

## how to install crab

- with crates.io:

```bash
cargo install crab
```

- from sources

```bash
cargo install --path /path/to/crab/repo/
```

- or just download zip or tar.gz from releases section

## how to use

- print DOM's tree

```bash
$ crab <url>
```

- print DOM's tree of specific tags

```bash
$ crab <url> get <css-selector>
```

- extra options:

```
 -e, --expend-url    printing expending url instead a relative path
 -h, --help          Prints help information
 -n, --no-colors     show DOM without colors
 -r, --row           print row content of tag
 -V, --version       Prints version information

```
