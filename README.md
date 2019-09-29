# Ambients _(ambients)_

> Peer-to-Peer Programs and Data

## Background

This repository is my attempt to test my understanding of the Ambients Protocol whitepaper by implementing it. At the moment, it
contains a parser that translates ambient syntax like ` a[in b] | b[in_ a]` to an AST structure that Rust work with.

## Install

First, [install Rust](https://www.rust-lang.org/tools/install) and install the rust nightly toolchain if you havent yet. Then:

```bash
$ git clone https://github.com/aphelionz/ambients
$ cd ambients
$ cargo build
```

# Usage

Currently there is no `main` function but you can see usage in, and also run, the tests.

```bash
% cargo 
```

# Contributing

Please do! If you're _at all_ interested in this topic you should definitely
[seek us out on Gitter](https://gitter.im/ambientsprotocol/community), open issues, and submit PRs.

# License

MIT
