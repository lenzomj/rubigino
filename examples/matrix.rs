use rubigino::display::matrix::{MAX72XX};
use rubigino::display::font::Character;
use rubigino::util;
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(500);

    let mut max: MAX72XX = MAX72XX::init().unwrap();
    max.clear().unwrap();

    for val in "Hello".chars() {
        let c: Character = Character::from(val);
        let mut input: [u8; 8] = c.encoding();
        util::rotate_cw(&mut input);
        max.write(&input).unwrap();
        thread::sleep(delay);
    }
}
