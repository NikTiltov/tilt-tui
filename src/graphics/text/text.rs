use crate::prelude::*;

use super::Line;

pub struct Text<'a> {
    lines: Vec<Line<'a>>,
    pub style: Style,
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(s.lines().map(Line::from).collect())
    }
}

impl<'a> From<String> for Text<'a> {
    fn from(s: String) -> Self {
        Self::new(s.lines().map(ToOwned::to_owned).map(Line::from).collect())
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Self::new(lines)
    }
}

impl<'a> Text<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        Self {
            lines,
            style: Style::default(),
        }
    }

    pub fn lines(&self) -> impl Iterator<Item = &Line<'a>> {
        self.lines.iter()
    }
}
