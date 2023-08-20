use super::{Color, Style};

#[derive(Clone, Copy)]
pub struct Cell {
    pub symbol: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    pub fn set_symbol(&mut self, symbol: char) {
        self.symbol = symbol;
    }

    pub fn set_fg(&mut self, color: Color) {
        self.fg = color;
    }

    pub fn set_bg(&mut self, color: Color) {
        self.bg = color;
    }

    pub fn set_style(&mut self, style: Style) {
        let Style { fg, bg } = style;
        if let Some(color) = fg {
            self.fg = color;
        }
        if let Some(color) = bg {
            self.bg = color;
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            symbol: value,
            ..Default::default()
        }
    }
}
