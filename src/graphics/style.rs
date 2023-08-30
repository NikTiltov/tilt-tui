use super::{CellMods, Color};

#[derive(Debug, Default, Clone, Copy)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub mods: CellMods,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn mods(mut self, mods: CellMods) -> Self {
        self.mods = mods;
        self
    }
}
