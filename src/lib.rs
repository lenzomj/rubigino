pub mod gpio;
pub mod parts;
pub mod spi;

pub trait Bits {
    fn to_bits(&self) -> [u8; 8];
    fn from_bits(bits: &[u8; 8]) -> u8;
}

impl Bits for u8 {
    fn to_bits(&self) -> [u8; 8] {
        let mut bits: [u8; 8] = [0; 8];
        for b in 0 .. 8 {
            bits[8 - b - 1] = (self >> b) & 0b00000001;
        }
        bits
    }

    fn from_bits(bits: &[u8; 8]) -> u8 {
        let byte: u8 = bits[0] << 7
            | bits[1] << 6
            | bits[2] << 5
            | bits[3] << 4
            | bits[4] << 3
            | bits[5] << 2
            | bits[6] << 1
            | bits[7] << 0;
        byte
    }
}

pub fn to_bit_matrix(bytes: &[u8; 8]) -> [[u8; 8]; 8] {
    let mut bit_matrix: [[u8; 8]; 8] = [[0; 8]; 8];
    for (b, byte) in bytes.iter().enumerate() {
        bit_matrix[b] = byte.to_bits();
    }
    bit_matrix
}

pub fn rotate_cw(bytes: &mut [u8; 8]) {
    let bit_matrix: [[u8; 8]; 8] = to_bit_matrix(bytes);
    let mut rot_matrix: [[u8; 8]; 8] = [[0; 8]; 8];
    for (r, byte) in bit_matrix.iter().enumerate() {
        for (c, _bit) in byte.iter().enumerate() {
            rot_matrix[c][8-r-1] = bit_matrix[r][c];
        }
    }
    for (b, byte) in rot_matrix.iter().enumerate() {
        bytes[b] = u8::from_bits(byte);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
