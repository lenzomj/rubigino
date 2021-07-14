#[cfg(feature = "rpi")]
pub mod rpi;

use std::fmt;

pub struct Bus<T> {
    bus_clk: u32,
    bus_dev: T,
}

pub enum SPIErr {
    IOError(String);
}

pub trait SPI<T> {
    fn init(clock_speed: u32) -> Result<Bus<T>, SPIErr>
    fn write(&mut self, bytes: &[u8]) -> Result<usize, SPIErr>;
}

impl<T> Bus<T> {
    pub fn new(clock_speed: u32, bus_dev: T) -> Bus<T> {
        Bus {
            bus_clk: clock_speed,
            bus_dev: bus_dev,
        }
    }
}

impl fmt::Debug for SPIErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SPIErr::IOError(string) => write!(f, "{ }", string)
        }
    }
}
