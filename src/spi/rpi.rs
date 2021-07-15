pub use crate::spi::{Bus, SPIErr, SPI};
pub use rppal::spi::Spi as RPiSpi;

use rppal::spi::{Bus as RPiBus, Mode, SlaveSelect};

impl SPI<RPiSpi> for Bus<RPiSpi> {
    fn init(clock_speed: u32) -> Result<Bus<RPiSpi>, SPIErr> {
        let spi = match RPiSpi::new(RPiBus::Spi0, SlaveSelect::Ss0, clock_speed, Mode::Mode0) {
            Ok(result) => result,
            Err(e) => {
                return Err(SPIErr::IOError(format!("{:?}", e)));
            }
        };

        return Ok(Bus::new(clock_speed, spi));
    }

    fn write(&mut self, bytes: &[u8]) -> Result<usize, SPIErr> {
        let n: usize = match self.bus_dev.write(bytes) {
            Ok(result) => result,
            Err(e) => {
                return Err(SPIErr::IOError(format!("{:?}", e)));
            }
        };
        return Ok(n);
    }
}
