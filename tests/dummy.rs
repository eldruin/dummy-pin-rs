use dummy_pin::DummyPin;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};

#[test]
fn can_create_low_and_read() {
    let mut pin = DummyPin::new_low();
    assert!(pin.is_low().unwrap());
    assert!(!pin.is_high().unwrap());
}

#[test]
fn can_create_high_and_read() {
    let mut pin = DummyPin::new_high();
    assert!(!pin.is_low().unwrap());
    assert!(pin.is_high().unwrap());
}

#[test]
fn can_set() {
    let mut pin = DummyPin::new_high();
    pin.set_high().unwrap();
    pin.set_low().unwrap();
}

#[test]
fn can_toggle() {
    let mut pin = DummyPin::new_low();
    pin.toggle().unwrap();
}

#[test]
fn can_read_state_set() {
    let mut pin = DummyPin::new_low();
    assert!(pin.is_set_low().unwrap());
    assert!(!pin.is_set_high().unwrap());
}
