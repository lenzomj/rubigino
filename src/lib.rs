pub mod gpio;
pub mod parts;
pub mod spi;

pub mod util {

    use std::convert::TryFrom;

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

    pub fn transpose(bytes: &mut [u8; 8]) {
        let mut x: u32 = (u32::from(bytes[0]) << 24)
            | (u32::from(bytes[1]) << 16)
            | (u32::from(bytes[2]) << 08)
            | (u32::from(bytes[3]));
        let mut y: u32 = (u32::from(bytes[4]) << 24)
            | (u32::from(bytes[5]) << 16)
            | (u32::from(bytes[6]) << 08)
            | (u32::from(bytes[7]));

        let mut t: u32 = (x ^ (x >> 7)) & 0x00aa00aa;
        x = x ^ t ^ (t << 7);
        t = (y ^ (y >> 7)) & 0x00aa00aa;
        y = y ^ t ^ (t << 7);

        t = (x ^ (x >> 14)) & 0x0000cccc;
        x = x ^ t ^ (t << 14);
        t = (y ^ (y >> 14)) & 0x0000cccc;
        y = y ^ t ^ (t << 14);

        t = (x & 0xf0f0f0f0) | ((y >> 4) & 0x0f0f0f0f);
        y = ((x << 4) & 0xf0f0f0f0) | (y & 0x0f0f0f0f);
        x = t;

        *bytes = [
            u8::try_from((x >> 24) & 0x000000ff).unwrap(),
            u8::try_from((x >> 16) & 0x000000ff).unwrap(),
            u8::try_from((x >> 8) & 0x000000ff).unwrap(),
            u8::try_from((x) & 0x000000ff).unwrap(),
            u8::try_from((y >> 24) & 0x000000ff).unwrap(),
            u8::try_from((y >> 16) & 0x000000ff).unwrap(),
            u8::try_from((y >> 8) & 0x000000ff).unwrap(),
            u8::try_from((y) & 0x000000ff).unwrap(),
        ];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
