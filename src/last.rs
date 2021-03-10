use core::convert::Infallible;
use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

/// Dummy pin implementation which stores the last level set to it and returns it when read.
///
/// Note that this implementation is not zero-cost due to the need to update the level dynamically.
#[derive(Debug, Clone, Copy)]
pub struct LastStateDummyPin {
    is_high: bool,
}

impl LastStateDummyPin {
    /// Create new instance of pin with the initial level provided.
    ///
    /// `true` corresponds to a high level and `false` to low level.
    pub fn new(is_high: bool) -> Self {
        LastStateDummyPin { is_high }
    }

    /// Create new instance of pin initially set low.
    pub fn new_low() -> Self {
        LastStateDummyPin { is_high: false }
    }

    /// Create new instance of pin initially set high.
    pub fn new_high() -> Self {
        LastStateDummyPin { is_high: true }
    }
}

impl OutputPin for LastStateDummyPin {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.is_high = true;
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.is_high = false;
        Ok(())
    }
}

impl InputPin for LastStateDummyPin {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high)
    }
}

impl ToggleableOutputPin for LastStateDummyPin {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.is_high = !self.is_high;
        Ok(())
    }
}

impl StatefulOutputPin for LastStateDummyPin {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high)
    }
}
