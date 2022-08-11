//! This provides dummy implementations of the input/output pin [`embedded-hal`] traits.
//! This is useful when dealing with setups where a certain pin is handled by hardware in a way
//! that the software does not need to know about, for example.
//!
//! In addition to the no-op, zero-cost `DummyPin`, this crate provides an implementation of `LastStateDummyPin`,
//! which stores the last state set at runtime and returns it when read.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! ## Usage examples
//!
//! ### `DummyPin` interaction
//! A `DummyPin` does nothing and always returns the creation level when read.
//!
//! ```
//! # use embedded_hal::digital::blocking::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
//! use dummy_pin::DummyPin;
//!
//! // This pin will always read low.
//! let mut pin = DummyPin::new_low();
//!
//! // Using pin as `OutpuPin` does nothing and returns `Ok(())`
//! pin.set_high().unwrap();
//!
//! // Using pin as `InputPin` always returns the creation level.
//! assert!( pin.is_low().unwrap() );
//! assert!( !pin.is_high().unwrap() );
//!
//! // Using pin as `ToggleableOutputPin` does nothing and returns `Ok(())`
//! pin.toggle().unwrap();
//!
//! // Using pin as `StatefulOutputPin` always returns the creation level.
//! assert!( pin.is_set_low().unwrap() );
//! assert!( !pin.is_set_high().unwrap() );
//! ```
//!
//! ### Pass dummy output pin to irrelevant driver output
//!
//! ```no_run
//! use dummy_pin::DummyPin;
//! use embedded_hal::digital::blocking::OutputPin;
//! use linux_embedded_hal::SysfsPin;
//!
//! struct Driver<P> {
//!     output: P,
//! }
//!
//! impl<P, E> Driver<P>
//! where
//!     P: OutputPin<Error = E>,
//! {
//!     fn new(pin: P) -> Self {
//!         Driver { output: pin }
//!     }
//!
//!     fn do_something(&mut self) -> Result<(), E> {
//!         // ...
//!         self.output.set_high()
//!     }
//! }
//!
//! // The same driver can operate with either a real or a dummy pin.
//! let real_pin = SysfsPin::new(25);
//! let mut driver_with_real_pin = Driver::new(real_pin);
//! driver_with_real_pin.do_something().unwrap();
//!
//! let dummy_pin = DummyPin::new_low();
//! let mut driver_with_dummy_pin = Driver::new(dummy_pin);
//! driver_with_dummy_pin.do_something().unwrap();
//! ```
//!
//! ### `LastStateDummyPin` interaction
//! A `LastStateDummyPin` stores the last level set and returns it when read.
//!
//! ```
//! # use embedded_hal::digital::blocking::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
//! use dummy_pin::LastStateDummyPin;
//!
//! // Initially this pin reads low.
//! let mut pin = LastStateDummyPin::new_low();
//!
//! assert!( pin.is_low().unwrap() );
//!
//! // Using pin as `OutpuPin` stores the level set
//! pin.set_high().unwrap();
//!
//! // Using pin as `InputPin` returns the last level.
//! assert!( pin.is_high().unwrap() );
//!
//! // Using pin as `ToggleableOutputPin` toggles the internal state
//! pin.toggle().unwrap();
//!
//! assert!( pin.is_low().unwrap() );
//!
//! // Using pin as `StatefulOutputPin` returns the last level.
//! assert!( pin.is_set_low().unwrap() );
//! assert!( !pin.is_set_high().unwrap() );
//! ```
//!

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod dummy;
pub use crate::dummy::level;
pub use crate::dummy::DummyPin;

mod last;
pub use crate::last::LastStateDummyPin;
