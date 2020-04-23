# Festive

[![docs](https://docs.rs/festive/badge.svg)](https://docs.rs/festive)
[![crates.io](https://img.shields.io/crates/v/festive.svg)](https://crates.io/crates/festive)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/clippy.svg)](#license)
![CI](https://github.com/estk/festive/workflows/CI/badge.svg)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42+-green.svg)](https://github.com/estk/festive#rust-version-requirements)

Festive is a celebration of the [`rusty_fork`](https://crates.io/crates/rusty-fork) crate. I have stripped down, modernized and proc_macroifyied our well loved `rusty_fork` crate as a learning experiment. What has come out on the other side is an extremely simple alternative where you may run your tests in an isolated process via the `#[festive]` attribute. No need to add the `#[test]` attr and if you would like a timeout just do something like the following: `#[festive(timeout_ms = 100)]`.

## Quality

If you want a crate that is battle hardened over the years, use `rusty_fork`. If you want something that has lots more tests use `rusty_fork`. If you want something that isnt a toy project use `rusty_fork`. If you want.... well you get the picture.

## Todo:

- [ ] Add failing test file for #should_panic

## Example

```rust
use festive::festive;
use std::process;

#[festive]
fn forked() {
    println!("Forked: My pid={}", process::id());
}
```

## Rust Version Requirements

1.42+

## License

Licensed under either of the following at your option.

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Credit

Most of this is lifted from `rusty_fork`, please direct any credit there.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
