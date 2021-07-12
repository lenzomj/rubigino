pub use crate::gpio::{Pin, PinMode, GPIO, GPIOErr};
pub use rppal::gpio::IoPin as RPiPin;


use rppal::gpio::{Gpio, Mode, Error};

impl GPIO<RPiPin> for Pin<RPiPin> {
    fn init(pin_num: u8) -> Result<Pin<RPiPin>, GPIOErr> {
        let gpio = match Gpio::new() {
            Ok(result) => result,
            Err(e) => {
                return Err(GPIOErr::IOError(format!("{:?}", e)));
            }
        };

        let raw_pin = match gpio.get(pin_num) {
            Ok(result) => result,
            Err(e) => {
                return Err(GPIOErr::IOError(format!("{:?}", e)));
            }
        };

        let io_pin = raw_pin.into_io(Mode::Input);

        return Ok(Pin::new(pin_num, io_pin));
    }

    fn is_low(&self) -> bool {
        self.pin_dev.is_low()
    }

    fn is_high(&self) -> bool {
        self.pin_dev.is_high()
    }

    fn set_mode(&mut self, mode: PinMode) -> Result<(), GPIOErr> {
        match mode {
            PinMode::Input  => self.pin_dev.set_mode(Mode::Input),
            PinMode::Output => self.pin_dev.set_mode(Mode::Output),
            _ => { }
        };
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
