use std::{thread, time};
use rubigino::gpio::rpi;

fn main() {
    let delay = time::Duration::from_millis(500);
    let mut pin = rpi::Pin<IoPin>::init(14)?;

    pin.to_output()?;
    loop {
        pin.toggle()?;
        thread::sleep(delay);
    }
}
