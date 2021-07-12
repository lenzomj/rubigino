use std::{thread, time};
use rubigino::gpio::rpi::{Pin, PinMode, GPIO, GPIOErr, RPiPin};

fn main() {
    let delay = time::Duration::from_millis(500);
    let mut pin: Pin<RPiPin> = Pin::init(14).unwrap();

    pin.set_mode(PinMode::Output);

    loop {
        pin.toggle();
        thread::sleep(delay);
    }
}
