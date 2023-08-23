use super::Color;

#[derive(Debug, Default, Clone, Copy)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
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

    pub fn merge(self, other: Self) -> Self {
        Self {
            fg: self.fg.or(other.fg),
            bg: self.bg.or(other.bg),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bg.is_none() && self.fg.is_none()
    }
}
