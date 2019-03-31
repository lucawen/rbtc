# RBTC

[![Build Status](https://travis-ci.com/lucawen/rbtc.svg?branch=master)](https://travis-ci.com/lucawen/rbtc)
[![Crates.io](https://img.shields.io/crates/v/rbtc.svg)](https://crates.io/crates/rbtc)

RBTC is cli to convert BTC to any currency and vice-versa.

<p align="center">
  <img width="100%" src="https://cdn.jsdelivr.net/gh/lucawen/rbtc/example.gif">
</p>


#### Building for source
For build the binary just:
```sh
$ cargo build
```
To run as debug, just run this example:
```sh
$ cargo run -- 1 --from USD --to BTC
```
### Installation
Install simple typing:

```sh
cargo install rbtc
```

### Documentation
The documentation, for now, is the help return of tool:

```sh
Get value of a btc value to a currency

USAGE:
    rbtc [FLAGS] [OPTIONS] <amount>

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Silent information abount currency result
    -V, --version    Prints version information
    -v, --verbose    Verbose errors

OPTIONS:
    -f, --from <from>    Set the initial currency of [default: BTC]
    -t, --to <to>        Set the final currency to convert [default: USD]

ARGS:
    <amount>    Set amount to convert to a currency or from a currency
```


License
----

MIT


**Free Software, Hell Yeah!**
