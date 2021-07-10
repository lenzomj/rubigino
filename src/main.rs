use rppal::spi::{Spi, Bus, SlaveSelect, Mode, Error};

const MAX72XX_TEST: u8 = 0x0F;
const MAX72XX_INTENSITY: u8 = 0x0A;
const MAX72XX_D0: u8 = 0x01;
const MAX72XX_D1: u8 = 0x02;
const MAX72XX_D2: u8 = 0x03;
const MAX72XX_D3: u8 = 0x04;
const MAX72XX_D4: u8 = 0x05;
const MAX72XX_D5: u8 = 0x06;
const MAX72XX_D6: u8 = 0x07;
const MAX72XX_D7: u8 = 0x08;
const MAX72XX_DECODE: u8 = 0x09;
const MAX72XX_SCANLIMIT: u8 = 0x0B;
const MAX72XX_SHUTDOWN: u8 = 0x0C;

const ROW_SIZE: u8  = 8;
const COL_SIZE: u8  = 8;
const MAX_INTENSITY: u8 = 0xf;
const MAX_SCANLIMIT: u8 = 7;


fn main() {
    let mut s = match Spi::new(Bus::Spi0,
                         SlaveSelect::Ss0,
                         10_000,
                         Mode::Mode0) {
        Ok(spi) => spi,
        Err(e) => panic!("{:?}", e)
    };

    s.write(&[MAX72XX_TEST, 0]).unwrap();
    s.write(&[MAX72XX_SCANLIMIT, ROW_SIZE - 1]);
    s.write(&[MAX72XX_INTENSITY, MAX_INTENSITY / 4]).unwrap();
    s.write(&[MAX72XX_DECODE, 0]).unwrap();
    s.write(&[MAX72XX_D0, 0b00000000]).unwrap();
    s.write(&[MAX72XX_D1, 0b00000000]).unwrap();
    s.write(&[MAX72XX_D2, 0b00000000]).unwrap();
    s.write(&[MAX72XX_D3, 0b00000000]).unwrap();
    s.write(&[MAX72XX_D4, 0b00000000]).unwrap();
    s.write(&[MAX72XX_D5, 0b00000000]).unwrap();
    s.write(&[MAX72XX_D6, 0b00000000]).unwrap();
    s.write(&[MAX72XX_D7, 0b00000000]).unwrap();
    s.write(&[MAX72XX_SHUTDOWN, 1]).unwrap();
    s.write(&[MAX72XX_D0, 0x30]).unwrap();
    s.write(&[MAX72XX_D1, 0x7c]).unwrap();
    s.write(&[MAX72XX_D2, 0xc0]).unwrap();
    s.write(&[MAX72XX_D3, 0x78]).unwrap();
    s.write(&[MAX72XX_D4, 0x0c]).unwrap();
    s.write(&[MAX72XX_D5, 0xf8]).unwrap();
    s.write(&[MAX72XX_D6, 0x30]).unwrap();
    s.write(&[MAX72XX_D7, 0x00]).unwrap();
}
