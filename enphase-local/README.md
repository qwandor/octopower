# enphase-local

[![crates.io page](https://img.shields.io/crates/v/enphase-local.svg)](https://crates.io/crates/enphase-local)
[![docs.rs page](https://docs.rs/enphase-local/badge.svg)](https://docs.rs/enphase-local)

`enphase-local` is a client library for the Enphase IQ Gateway
[local API](https://github.com/Matthew1471/Enphase-API/tree/main/Documentation).

This is not an officially supported Google product.

## Usage

To login and fetch production information:

```rust
use enphase_local::Envoy;
use reqwest::Url;

const AUTH_TOKEN: &str = "...";

let envoy = Envoy::new(Url::parse("https://envoy.local/")?, AUTH_TOKEN);
let production = envoy.production().await?;
```

## Example

For a more complete usage sample, see the included [example](examples/info.rs). To run it you'll
need the base URL of your local Enphase gateway, and an authentication token. You can get an
authentication token from https://entrez.enphaseenergy.com/, as described by
[Enphase's documentation](https://enphase.com/download/accessing-iq-gateway-local-apis-or-local-ui-token-based-authentication).

```
$ cargo run --example info https://envoy.local/ my_auth_token
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
