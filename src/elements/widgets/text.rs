use std::{slice::Iter, str::Chars};

use crate::{prelude::*, Widget};

pub fn text<'a>(txt: impl Into<Span<'a>>) -> Textbox<'a> {
    Textbox::new(vec![txt.into()])
}

pub fn text_spans<'a>(
    txt: impl IntoIterator<Item = impl Into<Span<'a>>>,
) -> Textbox<'a> {
    Textbox::new(txt.into_iter().map(Into::into).collect())
}

pub struct Textbox<'a> {
    spans: Vec<Span<'a>>,
    align_h: Align,
    align_v: Align,
    wrap: Wrap,
}

impl<'a> Textbox<'a> {
    pub fn new(spans: Vec<Span<'a>>) -> Self {
        Self {
            spans,
            align_h: Align::Start,
            align_v: Align::Start,
            wrap: Wrap::LetterWrap,
        }
    }

    pub fn align_h(mut self, align: Align) -> Self {
        self.align_h = align;
        self
    }

    pub fn align_v(mut self, align: Align) -> Self {
        self.align_v = align;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }
}

impl<'a> Widget for Textbox<'a> {
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        let text = self.wrap.apply(&self.spans, limits.max.w);
        let width = text.iter().map(|line| line.len()).max().unwrap_or(0);
        let height = text.len();
        Layout::new(limits.clamp(Size::new(width, height)))
    }

    fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        let rect = layout.rect();
        if rect.w * rect.h == 0 {
            return;
        }
        let text = self.wrap.apply(&self.spans, rect.w);
        let align_y = match self.align_v {
            Align::Start => 0,
            Align::Center => rect.h.saturating_sub(text.len()) / 2,
            Align::End => rect.h.saturating_sub(text.len()),
        };
        for (y, line) in text.into_iter().enumerate() {
            let align_x = match self.align_h {
                Align::Start => 0,
                Align::Center => rect.w.saturating_sub(line.len()) / 2,
                Align::End => rect.w.saturating_sub(line.len()),
            };
            for (x, (ch, style)) in line.into_iter().enumerate() {
                let i = rect.x + x + align_x;
                let j = rect.y + y + align_y;
                let cell = renderer.cell_mut(i, j);
                cell.ch = ch;
                cell.set_style(style);
            }
        }
    }
}

pub enum Wrap {
    NoWrap,
    LetterWrap,
    WordWrap,
}

impl Wrap {
    pub fn apply<'a>(
        &'a self,
        text: &'a [Span<'a>],
        width: usize,
    ) -> Vec<Vec<(char, Style)>> {
        let chars = StyledChars::new(text).collect::<Vec<_>>();
        let lines = chars.split(|(ch, _)| *ch == '\n').map(ToOwned::to_owned);
        match self {
            Wrap::NoWrap => lines.collect(),
            Wrap::LetterWrap => lines
                .flat_map(|line| {
                    line.chunks(width)
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .collect(),
            Wrap::WordWrap => todo!(),
        }
    }
}

struct StyledChars<'a> {
    spans: Iter<'a, Span<'a>>,
    chars: Chars<'a>,
    style: Style,
}

impl<'a> StyledChars<'a> {
    pub fn new(spans: &'a [Span<'a>]) -> Self {
        let mut spans = spans.iter();
        let span = spans.next().unwrap();
        Self {
            spans,
            chars: span.chars(),
            style: span.style(),
        }
    }
}

impl<'a> Iterator for StyledChars<'a> {
    type Item = (char, Style);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.chars.next() {
            Some((ch, self.style))
        } else {
            if let Some(span) = self.spans.next() {
                self.chars = span.chars();
                self.style = span.style();
                self.next()
            } else {
                None
            }
        }
    }
}
