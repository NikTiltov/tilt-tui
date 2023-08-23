use crate::prelude::*;
use std::borrow::Cow;

use super::Grapheme;

pub fn span<'a>(value: impl Into<Span<'a>>) -> Span<'a> {
    value.into()
}

pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
}

impl<'a> From<&'a str> for Span<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            content: value.into(),
            style: Style::default(),
        }
    }
}

impl<'a> From<String> for Span<'a> {
    fn from(value: String) -> Self {
        Self {
            content: value.into(),
            style: Style::default(),
        }
    }
}

impl<'a> Span<'a> {
    pub fn fg(mut self, color: Color) -> Self {
        self.style.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.style.bg = Some(color);
        self
    }

    pub fn len(&self) -> usize {
        self.content.chars().count()
    }

    pub fn chars(&'a self) -> impl Iterator<Item = Grapheme> + 'a {
        self.content.chars().map(|char| (char, self.style))
    }
}
