# Dummy Input/Output Pin Implementations

[![crates.io](https://img.shields.io/crates/v/dummy-pin.svg)](https://crates.io/crates/dummy-pin)
[![Docs](https://docs.rs/dummy-pin/badge.svg)](https://docs.rs/dummy-pin)
![MSRV](https://img.shields.io/badge/rustc-1.54+-blue.svg)
[![Build Status](https://github.com/eldruin/dummy-pin-rs/workflows/Build/badge.svg)](https://github.com/eldruin/dummy-pin-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/dummy-pin-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/dummy-pin-rs?branch=master)

This provides dummy implementations of the input/output pin [`embedded-hal`] traits.
This is useful when dealing with setups where a certain pin is handled by hardware in a way
that the software does not need to know about, for example.

In addition to the no-op, zero-cost `DummyPin`, this crate provides an implementation of `LastStateDummyPin`,
which stores the last state set at runtime and returns it when read.

## Usage

This example demonstrates how the same driver can operate with either a real or a dummy output pin.

```rust
use dummy_pin::DummyPin;
use embedded_hal::digital::OutputPin;
use linux_embedded_hal::SysfsPin;

struct Driver<P> {
    output: P,
}

impl<P, E> Driver<P>
where
    P: OutputPin<Error = E>,
{
    fn new(pin: P) -> Self {
        Driver { output: pin }
    }

    fn do_something(&mut self) -> Result<(), E> {
        // ...
        self.output.set_high()
    }
}

fn main() {
    // The same driver can operate with either a real or a dummy pin.
    let real_pin = SysfsPin::new(25);
    let mut driver_with_real_pin = Driver::new(real_pin);
    driver_with_real_pin.do_something().unwrap();

    let dummy_pin = DummyPin::new_low();
    let mut driver_with_dummy_pin = Driver::new(dummy_pin);
    driver_with_dummy_pin.do_something().unwrap();
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/dummy-pin-rs/issues).

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.54 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
