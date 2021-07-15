use rubigino::parts::MAX72XX;
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(500);

    let mut max: MAX72XX = MAX72XX::init().unwrap();
    max.clear().unwrap();

    let mut input: [u8; 8] = [
        0b_00000000,
        0b_00011000,
        0b_00011000,
        0b_11111111,
        0b_00011000,
        0b_00011000,
        0b_00011000,
        0b_00011000,
    ];

    loop {
        max.write(&input).unwrap();
        input = MAX72XX::rotate(&input);
        thread::sleep(delay);
    }
}
