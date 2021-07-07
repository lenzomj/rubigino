use rppal::gpio::{Gpio, IoPin, Mode, Error};
use std::{thread, time};

pub struct GPIOPin {
    num: u8,
    pin: IoPin,
}

impl GPIOPin {
    pub fn new(pin_num: u8) -> Option<GPIOPin> {
        let board = match Gpio::new() {
            Ok(result) => result,
            Err(_) => {
                println!("Error: Can't connect to GPIO.");
                return None;
            }
        };

        let unconfigured_pin = match board.get(pin_num) {
            Ok(result) => result,
            Err(Error::PinNotAvailable(pin)) => {
                println!("Error: Pin { } not available.", pin);
                return None;
            },
            Err(_) => {
                println!("Error: Unknown error.");
                return None;
            }
        };

        return Some(GPIOPin {
            num: pin_num,
            pin: unconfigured_pin.into_io(Mode::Input),
        });
    }

    pub fn set_input(&mut self) {
        self.pin.set_mode(Mode::Input);
    }

    pub fn set_output(&mut self) {
        self.pin.set_mode(Mode::Output);
    }

    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}

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
