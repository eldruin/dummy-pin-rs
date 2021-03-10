use dummy_pin::DummyPin;
use embedded_hal::digital::v2::OutputPin;
use linux_embedded_hal::Pin;

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
    let real_pin = Pin::new(25);
    let mut driver_with_real_pin = Driver::new(real_pin);
    driver_with_real_pin.do_something().unwrap();

    let dummy_pin = DummyPin::new_low();
    let mut driver_with_dummy_pin = Driver::new(dummy_pin);
    driver_with_dummy_pin.do_something().unwrap();
}
