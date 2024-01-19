use core::{convert::Infallible, marker::PhantomData};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

/// Pin level marker types for usage of `DummyPin` as an `InputPin` or `StatefulOutputPin`.
pub mod level {
    #[non_exhaustive]
    /// `DummyPin` will always behave as being high when checked.
    pub struct High;

    #[non_exhaustive]
    /// `DummyPin` will always behave as being low when checked.
    pub struct Low;
}

/// Dummy (no-op, zero-cost) pin
///
/// The implementation will discard any value written to it. When read,
/// it will always behave according to the value provided at construction
/// time (high or low).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DummyPin<L = level::Low> {
    _l: PhantomData<L>,
}

impl DummyPin<level::Low> {
    /// Create new instance
    ///
    /// When read it will always behave as being low.
    pub fn new_low() -> Self {
        DummyPin { _l: PhantomData }
    }
}

impl DummyPin<level::High> {
    /// Create new instance
    ///
    /// When read it will always behave as being high.
    pub fn new_high() -> Self {
        DummyPin { _l: PhantomData }
    }
}

impl<L> ErrorType for DummyPin<L> {
    type Error = Infallible;
}

impl<L> OutputPin for DummyPin<L> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl InputPin for DummyPin<level::Low> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

impl InputPin for DummyPin<level::High> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl StatefulOutputPin for DummyPin<level::Low> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

impl StatefulOutputPin for DummyPin<level::High> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}
