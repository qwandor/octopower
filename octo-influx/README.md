# octo-influx

[![crates.io page](https://img.shields.io/crates/v/octo-influx.svg)](https://crates.io/crates/octo-influx)

`octo-influx` is a tool to fetch smart meter reading data from the Octopus Energy API and import it
into an InfluxDB database. It also works for Octopus resellers such as London Power.

This is not an officially supported Google product.

## Installation

If you want to run `octo-influx` every night as a system service, you can install the latest release
from our Debian repository:

```sh
$ curl -L https://homiers.jfrog.io/artifactory/api/security/keypair/public/repositories/homie-rs | sudo apt-key add -
$ echo "deb https://homiers.jfrog.io/artifactory/homie-rs stable main" | sudo tee /etc/apt/sources.list.d/homie-rs.list
$ sudo apt update && sudo apt install octo-influx
```

Alternatively, you may install with cargo install:

```sh
$ cargo install octo-influx
```

## Usage

1. Create an InfluxDB database and grant some user write access to it.
2. Edit `/etc/octo-influx.toml` to add your account details and InfluxDB connection details.

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
