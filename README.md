# arsdk-rs   [![crates.io](https://img.shields.io/crates/v/arsdk-rs.svg)](https://crates.io/crates/arsdk-rs) [![Documentation](https://docs.rs/arsdk-rs/badge.svg)](https://docs.rs/arsdk-rs) [![MPL 2.0 License](https://img.shields.io/badge/license-apache2-green.svg)](LICENSE-APACHE) [![MIT License](https://img.shields.io/badge/license-mit-blue.svg)](LICENSE-MIT)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)



## jumpingsumo-rs   [![crates.io](https://img.shields.io/crates/v/jumpingsumo-rs.svg)](https://crates.io/crates/jumpingsumo-rs) [![Documentation](https://docs.rs/jumpingsumo-rs/badge.svg)](https://docs.rs/jumpingsumo-rs)

## bebop2

### Not released yet

## Sphinx simulator

https://developer.parrot.com/docs/sphinx/

1. Follow the installation guide

2. Simulate drone

```
sphinx /opt/parrot-sphinx/usr/share/sphinx/drones/{DRONE}.drone::stolen_interface={YOUR_INTERFACE}:eth0:192.168.42.1/24
```

Available drones:

```bash
$ ls /opt/parrot-sphinx/usr/share/sphinx/drones/
```
> airborne.drone  anafi4k.drone  bebop2.drone  bebop.drone  bluegrass.drone  disco.drone  mambo.drone  swing.drone

Run examples with IP:
```rust
use arsdk-rs::PARROT_SPHINX_IP;
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let expected = IpAddr::V4(Ipv4Addr::new(10, 202, 0, 1))
    assert_eq!(expected, PARROT_SPHINX_IP);
}
```



###


## Code of Conduct

We have a Code of Conduct so as to create a more enjoyable community and
work environment. Please see the [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md)
file for more details.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Dual MIT/Apache2 is strictly more permissive
