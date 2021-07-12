/*pub enum PinMode {
    Input,
    Output,
    InputOutput,
    NotSet
}

pub struct Pin<T> {
    pin_num: u8,
    pin_mod: PinMode,
    pin_dev: T
}

impl<T> Pin<T> {
    pub fn new(pin_num: u8, pin_dev: T) -> Pin<T> {
        Pin {
            pin_num: pin_num,
            pin_mod: PinMode::NotSet,
            pin_dev: pin_dev
        }
    }

    pub fn pin_num(&self) -> u8 {
        self.pin_num
    }

    pub fn is_input(&self) -> bool {
        match self.pin_mod {
            PinMode::Input => true,
            PinMode::InputOutput => true,
            _ => false
        }
    }

    pub fn is_output(&self) -> bool {
        match self.pin_mod {
            PinMode::Output => true,
            PinMode::InputOutput => true,
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
    fn set_mode(&mut self, mode: PinMode) -> Result<(), GPIOErr>;
    fn set_low(&mut self) -> Result<(), GPIOErr>;
    fn set_high(&mut self) -> Result<(), GPIOErr>;
    fn toggle(&mut self) -> Result<(), GPIOErr>;
}*/
