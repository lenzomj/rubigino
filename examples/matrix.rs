use rubigino::display::matrix::{MAX72XX};
use rubigino::display::font::Character;
use rubigino::util;
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(250);

    let mut max: MAX72XX = MAX72XX::init().unwrap();
    max.clear().unwrap();

    let input: String = format!(" { } ", "Hello");

    for val in input.chars().rev() {
        let c: Character = Character::from(val);
        let mut input: [u8; 8] = c.encoding();
        util::rotate_cw(&mut input);
        for shift in 1 .. 7 {
            util::shift_right(&mut input);
            max.write(&input).unwrap();
            thread::sleep(delay);
        }
    }
}
