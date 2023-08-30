use super::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub mods: CellMods,
}

impl Cell {
    pub fn set_style(&mut self, style: Style) {
        if let Some(color) = style.fg {
            self.fg = color;
        }
        if let Some(color) = style.bg {
            self.bg = color;
        }
        if !style.mods.is_empty() {
            self.mods = style.mods;
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
            mods: CellMods::empty(),
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            ch: value,
            ..Default::default()
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct CellMods: u8 {
        const Bold        = 0b0000_0001;
        const Italic      = 0b0000_0010;
        const Underlined  = 0b0000_0100;
        const Undercurled = 0b0000_1000;
        const Underdotted = 0b0001_0000;
        const CrossedOut  = 0b0010_0000;
        const Reverse     = 0b0100_0000;
    }
}
