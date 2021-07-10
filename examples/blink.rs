use std::{thread, time};
use rubigino::gpio::GPIOPin;

fn main() {
    let delay = time::Duration::from_millis(500);
    let mut pin = match GPIOPin::new(14) {
        Some(result) => result,
        None => {
            return;
        }
    };

    pin.set_output();
    loop {
        pin.toggle();
        thread::sleep(delay);
    }
}
