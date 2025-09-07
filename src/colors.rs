pub struct Color {
    pub c: u32,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            c: (Self::rgb2c(r, g, b)),
        }
    }

    pub fn rgb2c(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
}
