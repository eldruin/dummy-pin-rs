use core::{convert::Infallible, marker::PhantomData};
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

/// Pin level marker types for usage of `DummyPin` as an `InputPin` or `StatefulOutputPin`.
pub mod level {
    /// `DummyPin` will always behave as being high when checked.
    pub struct High;
    /// `DummyPin` will always behave as being low when checked.
    pub struct Low;
}

/// Dummy (no-op, zero-cost) pin
///
/// The implementation will discard any value written to it. When read,
/// it will always behave according to the value provided at construction
/// time (high or low).
#[derive(Debug, Clone, Copy)]
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

impl<L> OutputPin for DummyPin<L> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl InputPin for DummyPin<level::Low> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

impl InputPin for DummyPin<level::High> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl<L> ToggleableOutputPin for DummyPin<L> {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl StatefulOutputPin for DummyPin<level::Low> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

impl StatefulOutputPin for DummyPin<level::High> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}
