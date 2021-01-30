# Hathor file generator

Allows to quckly generate a set of similar files.

Named after [Hathor](https://en.wikipedia.org/wiki/Hathor) - goddess in ancient Egyptian religion.

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

*This is now in progress project. Things are broken. No warranty provided.*

Currently supported:

- generate set of same size files with predefined content based on string `0123456789`

## Usage

### Installation

```bash
cargo install hathor
```

### Generate files

To generate 5 files with the size 15 in directory `test` use:

```bash
$ mkdir test
$ hathor generate 15 5 test
$ ls test/
0 1 2 3 4
$ cat test/0
012345678901234
```

### Help

Check `--help` for future usage information.

```bash
$ hathor --help
hathor 0.1.0
Hathor - a file generator

USAGE:
    hathor <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate    Generates a set of same size files with repeating pattern `0123456789`
    help        Prints this message or the help of the given subcommand(s)
```
