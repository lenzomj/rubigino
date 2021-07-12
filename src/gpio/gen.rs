use crate::gpio::{Pin, PinMode, GPIO, GPIOErr};

pub struct GenIoPin {
    pub pin_state: bool
}

impl GPIO<GenIoPin> for Pin<GenIoPin> {
    fn init(pin_num: u8) -> Result<Pin<GenIoPin>, GPIOErr> {
        let genio = GenIoPin { pin_state: false };
        Ok(Pin::new(pin_num, genio))
    }

    fn is_low(&self) -> bool {
        !self.pin_dev.pin_state
    }

    fn is_high(&self) -> bool {
        self.pin_dev.pin_state
    }

    fn set_mode(&mut self, mode: PinMode) -> Result<(), GPIOErr> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), GPIOErr> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), GPIOErr> {
        Ok(())
    }

    fn toggle(&mut self) -> Result<(), GPIOErr> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::gpio::{Pin, GPIO};
    use crate::gpio::gen::{GenIoPin};

    #[test]
    fn it_works() {
        let mut pin: Pin<GenIoPin> = Pin::init(14).unwrap();
        assert_eq!(2 + 2, 4);
    }
}
