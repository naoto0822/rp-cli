# rp-cli

[![CICD](https://github.com/naoto0822/rp-cli/actions/workflows/cicd.yml/badge.svg)](https://github.com/naoto0822/rp-cli/actions/workflows/cicd.yml)
![License](https://img.shields.io/crates/l/rp-cli)
![Latest Version](https://img.shields.io/crates/v/rp-cli)

rp-cli is a cli of The Rust Playground.

## Installation

### From Source

```bash
$ cargo install rp-cli
```

### From Binaries

The [release page](https://github.com/naoto0822/rp-cli/releases) includes precompiled binaries.

## Usage

### Run

```bash
$ rp run ./examples/run.rs
```

### Format

```bash
$ rp fmt ./examples/run.rs
```

### Share

rp output "Permalink to the playground", "Direct link to the gist".

```bash
$ rp share ./examples/run.rs
```

### Download

rp can handle gist_id or rust playground url.

```bash
$ rp download 234b1b3dd6bf3c13ec78ea86db21e2cd
or
$ rp download https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=234b1b3dd6bf3c13ec78ea86db21e2cd
```

## Features

- [ ] output file (fmt, download)
- [ ] output JSON format
- [ ] more test code...
- [ ] declear custom error type

## Dependencies

- [clap-rs/clap](https://github.com/clap-rs/clap)
- [serde-rs/serde](https://github.com/serde-rs/serde)
- [serde-rs/json](https://github.com/serde-rs/json)
- [seanmonstar/reqwest](https://github.com/seanmonstar/reqwest)
- [servo/rust-url](https://github.com/servo/rust-url)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
