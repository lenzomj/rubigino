pub use crate::spi::{SPIErr, Bus};
pub use rppal::spi::Spi as RPiSpi;

use rppal::spi::{Bus, SlaveSelect, Mode};

impl SPI<RpiSpi> for Bus<RpiSpi> {
    fn init(clock_speed: u32) -> Result<Bus<T>, SPIErr> {
        let spi = match Spi::new(Bus::Spi0,
                                 SlaveSelect::Ss0,
                                 clock_speed,
                                 Mode::Mode0) {
            Ok(result) => result,
            Err(e) => {
                return Err(SPIErr::IOError(format!("{:?}", e)));
            }
        };

        return Ok(Bus::new(clock_speed, spi));
    }

    fn write(&mut self, bytes: &[u8]) -> Result<usize, SPIErr> {
        let n: usize = match self.bus_dev.write(bytes) {
            Ok(result) =>  result,
            Err(e) => {
                return Err(SPIErr::IOError(format!("{:?}", e)));
            }
        };
        return Ok(n);
    }
}
