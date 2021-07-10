use crate::gpio::GPIOPin;

pub struct DotMatrix {
    din_pin: GPIOPin,
    clk_pin: GPIOPin,
    cs_pin:  GPIOPin,
}

impl DotMatrix {
    pub fn new(din_pin_num: u8,
               clk_pin_num: u8,
               cs_pin_num: u8) -> Option<DotMatrix> {
        let mut dot_matrix = DotMatrix {
            din_pin: GPIOPin::new(din_pin_num)?,
            clk_pin: GPIOPin::new(clk_pin_num)?,
            cs_pin:  GPIOPin::new(cs_pin_num)?,
        };

        dot_matrix.din_pin.set_output();
        dot_matrix.clk_pin.set_output();
        dot_matrix.cs_pin.set_output();
        return Some(dot_matrix);
    }
}
