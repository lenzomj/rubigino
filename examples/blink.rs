use std::{thread, time};
use rubigino::gpio::rpi;

fn main() {
    let delay = time::Duration::from_millis(500);
    let mut pin: Pin = rpi::Pin::new(14).unwrap();

    pin.to_output()?;
    loop {
        pin.toggle()?;
        thread::sleep(delay);
    }
}
