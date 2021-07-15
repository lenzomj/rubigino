use rubigino::parts::MAX72XX;
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(500);

    let mut max: MAX72XX = MAX72XX::init(8, 8).unwrap();
    max.clear().unwrap();

    let input: [u8; 8] = [
        0b_10000010,
        0b_01000001,
        0b_10010010,
        0b_01101001,
        0b_10010010,
        0b_01000001,
        0b_10000010,
        0b_01000001,
    ];

    max.write(&input).unwrap();

    loop {
        thread::sleep(delay);
    }
}
