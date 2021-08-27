# cargo-ver

`cargo-ver` is a [cargo][] extension to manage crate versions.

# Usage

This command should always be invoked from [cargo][].

```sh

# show the current version
$ cargo ver
0.1.0
# set the crate version to 0.2.5
$ cargo ver set 0.2.5
updated version: 0.1.0 -> 0.2.5
# bump the major version
# short: cargo ver b maj
$ cargo ver bump major
updated version: 0.2.5 -> 1.0.0
```

# Installation

## Option 1: From crates.io

`cargo install --locked cargo-ver`

## Option 2: Clone the project


```sh
git clone github.com/insomnimus/cargo-ver
cd cargo-ver
git checkout main
cargo install --locked --path .
```

[cargo]: https://github.com/rust-lang/cargo
