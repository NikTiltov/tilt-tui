use crate::prelude::*;

use super::{Grapheme, Span};

pub fn line<'a>(value: impl Into<Line<'a>>) -> Line<'a> {
    value.into()
}

pub struct Line<'a> {
    pub spans: Vec<Span<'a>>,
    pub style: Style,
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(vec![Span::from(s)])
    }
}

impl<'a> From<String> for Line<'a> {
    fn from(s: String) -> Self {
        Self::new(vec![Span::from(s)])
    }
}

impl<'a> From<Vec<Span<'a>>> for Line<'a> {
    fn from(spans: Vec<Span<'a>>) -> Self {
        Self::new(spans)
    }
}

impl<'a> Line<'a> {
    pub fn new(spans: Vec<Span<'a>>) -> Self {
        Self {
            spans,
            style: Style::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.spans.iter().map(|span| span.len()).sum()
    }

    pub fn spans(&self) -> impl Iterator<Item = &Span<'a>> {
        self.spans.iter()
    }

    pub fn chars(&'a self) -> impl Iterator<Item = Grapheme> + 'a {
        self.spans
            .iter()
            .map(|span| {
                span.chars()
                    .map(|(char, style)| (char, style.merge(self.style)))
                    .collect::<Vec<_>>()
            })
            .flatten()
    }
}
