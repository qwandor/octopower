# octopower

[![crates.io page](https://img.shields.io/crates/v/octopower.svg)](https://crates.io/crates/octopower)
[![docs.rs page](https://docs.rs/octopower/badge.svg)](https://docs.rs/octopower)

A client library for a subset of the Octopus Energy API. This also works for Octopus resellers such
as London Power.

This is not an officially supported Google product.

## Usage

To try the included example, you'll need your email address, password and account ID. Your account
ID should be something like "A-1234ABCD".

```
$ cargo run --example readings email@address.domain mypassword A-1234ABCD
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

If you want to contribute to the project, see details of
[how we accept contributions](CONTRIBUTING.md).
