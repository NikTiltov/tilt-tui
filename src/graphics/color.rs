#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const BROWN: Self = Self::rgb(255, 255, 0);
    pub const VIOLET: Self = Self::rgb(255, 0, 255);
    pub const CYAN: Self = Self::rgb(0, 255, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn hex(hex: u32) -> Self {
        let r = (hex / 0x10000) as u8;
        let g = (hex / 0x100 % 0x100) as u8;
        let b = (hex % 0x100) as u8;
        Self { r, g, b }
    }
}
