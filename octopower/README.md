# octopower

[![crates.io page](https://img.shields.io/crates/v/octopower.svg)](https://crates.io/crates/octopower)
[![docs.rs page](https://docs.rs/octopower/badge.svg)](https://docs.rs/octopower)

`octopower` is a client library for a subset of the
[Octopus Energy API](https://developer.octopus.energy/docs/api/). This also works for Octopus
resellers such as London Power.

This is not an officially supported Google product.

## Usage

To login and fetch account information:

```rust
use octopower::{authenticate, get_account};

let token = authenticate("email@address.example", "password").await?;
let account = get_account(&token, "A-1234ABCD").await?;
println!("Account information: {:?}", account);
```

## Example

For a more complete usage sample, see the included [example](examples/readings.rs). To run it you'll
need your email address, password and account ID. Your account ID should be something like
"A-1234ABCD".

```
$ cargo run --example readings email@address.domain mypassword A-1234ABCD
```

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

If you want to contribute to the project, see details of
[how we accept contributions](../CONTRIBUTING.md).
