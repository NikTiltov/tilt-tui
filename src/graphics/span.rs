use crate::graphics::{Color, Style};
use std::{borrow::Cow, str::Chars};

use super::CellMods;

pub struct Span<'a> {
    content: Cow<'a, str>,
    style: Style,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for Span<'a> {
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            style: Style::new(),
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

    pub fn mods(mut self, mods: CellMods) -> Self {
        self.style.mods = mods;
        self
    }

    pub fn len(&self) -> usize {
        self.content.chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn chars(&'a self) -> Chars {
        self.content.chars()
    }

    pub fn str(&'a self) -> &str {
        &self.content
    }

    pub fn style(&self) -> Style {
        self.style
    }
}
