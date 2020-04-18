# Festive

[![docs](https://docs.rs/festive/badge.svg)](https://docs.rs/festive)
[![crates.io](https://img.shields.io/crates/v/festive.svg)](https://crates.io/crates/festive)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/clippy.svg)](#license)
![CI](https://github.com/estk/festive/workflows/CI/badge.svg)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42+-green.svg)](https://github.com/estk/festive#rust-version-requirements)

Festive is a celebration of the [`rusty_fork`](https://crates.io/crates/rusty-fork) crate. This provides an attribute `#[festive]` that will run a function as a test in a separate process. The `festive` attribute does exactly what the `rusty_fork_test` macro does to each containing function.

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

Most of this is lifted from `rusty_fork` and we use rusty_fork as a dep too, so credit should be directed there.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
