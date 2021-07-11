enum PinMode {
    Input,
    Output
}

pub struct Pin<T> {
    pin_num: u8,
    pin_mod: PinMode,
    pin_dev: Option<T>
}

impl<T> Pin<T> {
    pub fn pin_num(&self) -> u8 {
        self.pin_num
    }

    pub fn is_input(&self) -> bool {
        match self.pin_mod {
            PinMode::Input => true,
            _ => false
        }
    }

    pub fn is_output(&self) -> bool {
        match self.pin_mod {
            PinMode::Output => true,
            _ => false
        }
    }
}

pub enum GPIOErr {
    IOError(String),
    ModeError(String)
}

pub trait GPIO<T> {
    fn init(pin_num: u8) -> Result<Pin<T>, GPIOErr>;
    fn is_low(&self) -> bool;
    fn is_high(&self) -> bool;
    fn to_input(&mut self) -> Result<(), GPIOErr>;
    fn to_output(&mut self) -> Result<(), GPIOErr>;
    fn set_low(&mut self) -> Result<(), GPIOErr>;
    fn set_high(&mut self) -> Result<(), GPIOErr>;
    fn toggle(&mut self) -> Result<(), GPIOErr>;
}
