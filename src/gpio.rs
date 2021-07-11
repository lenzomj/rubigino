use crate::hal::{Pin, GPIO, GPIOErr};


#[cfg(all(target_arch="armv7"))]
mod rpi {
    use rppal::gpio::{Gpio, IoPin, Mode, Error};

    impl GPIO<IoPin> for Pin<IoPin> {

        fn init(pin_num: u8) -> Result<Pin<IoPin>, GPIOErr> {
            let board = match Gpio::new() {
                Ok(result) => result,
                Err(_) => {
                    return GPIOErr::IOError("Can't connect to GPIO");
                }
            };

            let unconfigured_pin = match board.get(pin_num) {
                Ok(result) => result,
                Err(Error::PinNotAvailable(pin)) => {
                    return GPIOErr::IOError(format!("Pin { } not available", pin));
                },
                Err(_) => {
                    return GPIOErr::IOError("Unknown error.");
                }
            };

            return Ok(Pin {
                pin_num: pin_num,
                pin_mod: PinMode::Input,
                pin_dev: unconfigured_pin.into_io(Mode::Input),
            });
        }

        fn is_low(&self) -> bool {
            self.pin_dev.is_low()
        }

        fn is_high(&self) -> bool {
            self.pin_dev.is_high()
        }

        fn to_input(&mut self) -> Result<(), GPIOErr> {
            self.pin_dev.set_mode(Mode::Input);
            return Ok(());
        }

        fn to_output(&mut self) -> Result<(), GPIOErr> {
            self.pin_dev.set_mode(Mode::Output);
            return Ok(());
        }

        fn set_low(&mut self) -> Result<(), GPIOErr> {
            self.pin_dev.set_low();
            return Ok(());
        }

        fn set_high(&mut self) -> Result<(), GPIOErr> {
            self.pin_dev.set_high();
            return Ok(());
        }

        fn toggle(&mut self) -> Result<(), GPIOErr> {
            self.pin_dev.toggle();
            return Ok(());
        }
    }
}
