

const BYTE_LIMIT: u16 = 256;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn parse(value: u32) -> Color {
        let limit = BYTE_LIMIT as u32;
        Color {
            r: (value / limit.pow(2) % limit) as u8,
            g: (value / limit % limit.pow(2)) as u8,
            b: (value % limit.pow(3)) as u8,
        }
    }
}
