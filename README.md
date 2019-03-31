# RBTC

[![Build Status](https://travis-ci.com/lucawen/rbtc.svg?branch=master)](https://travis-ci.com/lucawen/rbtc)

RBTC is cli to convert BTC to any currency and vice-versa.

<p align="center">
  <img width="600" src="https://cdn.jsdelivr.net/gh/lucawen/rbtc/example.svg">
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
rbtc 0.1.1
Luca Lacerda <lucalacerda1@gmail.com>
Get value of a btc value to a currency

USAGE:
    rbtc [FLAGS] [OPTIONS] <amount>

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Verbose information not will displayed
    -V, --version    Prints version information

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
