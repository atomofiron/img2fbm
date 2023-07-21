use std::ops::{Shl, Shr};


#[derive(Hash)]
pub struct Bitmap {
    pub width: u8,
    pub height: u8,
    pub bytes: Vec<u8>,
}

impl Bitmap {

    pub fn new(width: u8, height: u8) -> Bitmap {
        Bitmap { width, height, bytes: vec![0x00] }
    }

    pub fn set(&mut self, x: u32, y: u32) {
        let (byte, bit) = self.get_indexes(x, y);
        while self.bytes.len() <= byte {
            self.bytes.push(0x00)
        }
        let bit: u8 = 1u8.shl(bit);
        self.bytes[byte] |= bit;
    }

    pub fn invert(&mut self) {
        for index in 1..self.bytes.len() {
            self.bytes[index] = !self.bytes[index];
        }
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        let (byte, bit) = self.get_indexes(x, y);
        return self.bytes.get(byte)
            .map(|it| (it.shr(bit) & 1u8) == 1)
            .unwrap_or(false);
    }

    fn get_indexes(&self, x: u32, y: u32) -> (usize, usize) {
        let offset = (self.width as u32 * y + x) as usize;
        ((offset / 8 + 1), (7 - offset % 8))
    }
}
