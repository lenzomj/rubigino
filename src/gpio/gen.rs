use crate::gpio::{GPIOErr, Pin, PinMode, GPIO};

pub struct GenIoPin {
    pub pin_state: bool,
    pub pin_mode: PinMode,
}

impl GenIoPin {
    fn set_low(&mut self) {
        self.pin_state = false;
    }

    fn set_high(&mut self) {
        self.pin_state = true;
    }

    fn toggle(&mut self) {
        self.pin_state ^= true;
    }
}

impl GPIO<GenIoPin> for Pin<GenIoPin> {
    fn init(pin_num: u8) -> Result<Pin<GenIoPin>, GPIOErr> {
        let genio = GenIoPin {
            pin_state: false,
            pin_mode: PinMode::Input,
        };
        Ok(Pin::new(pin_num, genio))
    }

    fn is_low(&self) -> bool {
        !self.pin_dev.pin_state
    }

    fn is_high(&self) -> bool {
        self.pin_dev.pin_state
    }

    fn set_mode(&mut self, mode: PinMode) -> Result<(), GPIOErr> {
        self.pin_dev.pin_mode = mode;
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), GPIOErr> {
        self.pin_dev.set_low();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), GPIOErr> {
        self.pin_dev.set_high();
        Ok(())
    }

    fn toggle(&mut self) -> Result<(), GPIOErr> {
        self.pin_dev.toggle();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::gpio::gen::GenIoPin;
    use crate::gpio::{Pin, GPIO};

    fn setup(pin_num: u8) -> Pin<GenIoPin> {
        let pin: Pin<GenIoPin> = Pin::init(pin_num).unwrap();
        pin
    }

    #[test]
    fn test_init() {
        let pin: Pin<GenIoPin> = setup(14);
        assert_eq!(pin.pin_num(), 14);
    }

    #[test]
    fn test_set_high() {
        let mut pin: Pin<GenIoPin> = setup(14);
        pin.set_high().unwrap();
        assert_eq!(pin.is_high(), true);
    }
}
