use rubigino::gpio::rpi::{Pin, PinMode, RPiPin, GPIO};
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(500);
    let mut pin: Pin<RPiPin> = Pin::init(14).unwrap();

    pin.set_mode(PinMode::Output).unwrap();

    loop {
        pin.toggle().unwrap();
        thread::sleep(delay);
    }
}
