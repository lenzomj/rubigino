#[cfg(feature = "rpi")]
pub mod rpi {
    use crate::spi::rpi::{Bus, RPiSpi, SPIErr, SPI};


    const MAX72XX_TEST: u8 = 0x0F;
    const MAX72XX_INTENSITY: u8 = 0x0A;
    const MAX72XX_DECODE: u8 = 0x09;
    const MAX72XX_SCANLIMIT: u8 = 0x0B;
    const MAX72XX_SHUTDOWN: u8 = 0x0C;

    const MAX72XX_D0: u8 = 0x01;
    const MAX72XX_D1: u8 = 0x02;
    const MAX72XX_D2: u8 = 0x03;
    const MAX72XX_D3: u8 = 0x04;
    const MAX72XX_D4: u8 = 0x05;
    const MAX72XX_D5: u8 = 0x06;
    const MAX72XX_D6: u8 = 0x07;
    const MAX72XX_D7: u8 = 0x08;
    const MAX72XX_DX: [u8; 8] = [
        MAX72XX_D0, MAX72XX_D1, MAX72XX_D2, MAX72XX_D3, MAX72XX_D4, MAX72XX_D5, MAX72XX_D6, MAX72XX_D7,
    ];

    const MAX_INTENSITY: u8 = 0xf;

    pub struct MAX72XX {
        spi0_bus: Bus<RPiSpi>,
        row_size: u8,
        col_size: u8,
    }

    impl MAX72XX {
        pub fn init() -> Result<MAX72XX, SPIErr> {
            let mut max = MAX72XX {
                spi0_bus: Bus::init(10_000).unwrap(),
                row_size: 8,
                col_size: 8,
            };

            max.setup().unwrap();
            max.clear().unwrap();
            Ok(max)
        }

        pub fn clear(&mut self) -> Result<(), SPIErr> {
            for register in MAX72XX_DX {
                self.spi0_bus.write(&[register, 0x00]).unwrap();
            }
            self.spi0_bus.write(&[MAX72XX_SHUTDOWN, 1]).unwrap();
            Ok(())
        }

        pub fn write(&mut self, bytes: &[u8]) -> Result<(), SPIErr> {
            for (register, byte) in MAX72XX_DX.iter().zip(bytes) {
                self.spi0_bus.write(&[*register, *byte]).unwrap();
            }
            Ok(())
        }

        fn setup(&mut self) -> Result<(), SPIErr> {
            self.spi0_bus.write(&[MAX72XX_TEST, 0x00]).unwrap();
            self.spi0_bus
                .write(&[MAX72XX_SCANLIMIT, self.row_size - 1])
                .unwrap();
            self.spi0_bus
                .write(&[MAX72XX_INTENSITY, MAX_INTENSITY / 4])
                .unwrap();
            self.spi0_bus.write(&[MAX72XX_DECODE, 0x00]).unwrap();
            Ok(())
        }
    }
}
