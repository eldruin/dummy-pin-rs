use dummy_pin::LastStateDummyPin;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};

#[test]
fn can_create_low_and_read() {
    let mut pin = LastStateDummyPin::new_low();
    assert!(pin.is_low().unwrap());
    assert!(!pin.is_high().unwrap());
}

#[test]
fn can_create_high_and_read() {
    let mut pin = LastStateDummyPin::new_high();
    assert!(!pin.is_low().unwrap());
    assert!(pin.is_high().unwrap());
}

#[test]
fn can_create_with_value_and_read() {
    let mut pin = LastStateDummyPin::new(true);
    assert!(pin.is_high().unwrap());
}

#[test]
fn setting_changes_read_value() {
    let mut pin = LastStateDummyPin::new_low();
    assert!(pin.is_low().unwrap());
    pin.set_high().unwrap();
    assert!(pin.is_high().unwrap());
    pin.set_low().unwrap();
    assert!(pin.is_low().unwrap());
}

#[test]
fn can_toggle() {
    let mut pin = LastStateDummyPin::new_low();
    assert!(pin.is_low().unwrap());
    pin.toggle().unwrap();
    assert!(pin.is_high().unwrap());
}

#[test]
fn can_read_state_set() {
    let mut pin = LastStateDummyPin::new_low();
    assert!(pin.is_set_low().unwrap());
    pin.set_high().unwrap();
    assert!(pin.is_set_high().unwrap());
}
