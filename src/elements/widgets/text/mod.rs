mod line;
mod span;
mod text;

pub use line::{line, Line};
pub use span::{span, Span};
pub use text::Text;

pub type Grapheme = (char, crate::graphics::Style);

use crate::{prelude::*, Widget};

pub fn text<'a>(txt: impl Into<Text<'a>>) -> Textbox<'a> {
    Textbox::new(txt)
}

pub struct Textbox<'a> {
    content: Text<'a>,
    align_h: Alignment,
    align_v: Alignment,
    size: Size<Length>,
    wrap: Wrap,
}

impl<'a> Textbox<'a> {
    pub fn new(txt: impl Into<Text<'a>>) -> Self {
        Self {
            content: txt.into(),
            align_h: Alignment::Start,
            align_v: Alignment::Start,
            size: Size::min(),
            wrap: Wrap::LetterWrap,
        }
    }

    pub fn align_h(mut self, align: Alignment) -> Self {
        self.align_h = align;
        self
    }

    pub fn align_v(mut self, align: Alignment) -> Self {
        self.align_v = align;
        self
    }

    pub fn size(
        mut self,
        width: impl Into<Length>,
        height: impl Into<Length>,
    ) -> Self {
        self.size = Size::new(width.into(), height.into());
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.content.style = style;
        self
    }
}

impl<'a> Widget for Textbox<'a> {
    fn size(&self) -> Size<Length> {
        self.size
    }

    fn layout(&self, bound: Size) -> Layout {
        let text = if self.size.w == Length::Min || self.size.h == Length::Min {
            let width = self.size.w.var().unwrap_or(bound.w);
            self.wrap.apply(&self.content, width)
        } else {
            Vec::new()
        };
        let width = match self.size.w {
            Length::Var(var) => var,
            Length::Max => bound.w,
            Length::Min => text.iter().map(|line| line.len()).max().unwrap(),
        };
        let height = match self.size.h {
            Length::Var(var) => var,
            Length::Max => bound.h,
            Length::Min => text.len(),
        };
        Layout::new(Size::new(width, height))
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        let rect = layout.rect();
        let text = self.wrap.apply(&self.content, rect.w);
        let align_y = match self.align_v {
            Alignment::Start => 0,
            Alignment::Center => rect.h.saturating_sub(text.len()) / 2,
            Alignment::End => rect.h.saturating_sub(text.len()),
        };
        for (y, line) in text.into_iter().enumerate() {
            let align_x = match self.align_h {
                Alignment::Start => 0,
                Alignment::Center => rect.w.saturating_sub(line.len()) / 2,
                Alignment::End => rect.w.saturating_sub(line.len()),
            };
            for (x, (char, style)) in line.into_iter().enumerate() {
                let i = rect.x + x + align_x;
                let j = rect.y + y + align_y;
                let cell = canvas.cell_mut(i, j);
                cell.set_symbol(char);
                cell.set_style(style.merge(self.content.style));
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
        &self,
        text: &Text<'a>,
        width: usize,
    ) -> Vec<Vec<Grapheme>> {
        match self {
            Wrap::NoWrap => {
                text.lines().map(|line| line.chars().collect()).collect()
            }
            Wrap::LetterWrap => text
                .lines()
                .map(|line| -> Vec<Vec<Grapheme>> {
                    line.chars()
                        .collect::<Vec<_>>()
                        .chunks(width)
                        .map(ToOwned::to_owned)
                        .collect()
                })
                .flatten()
                .collect(),
            Wrap::WordWrap => todo!(),
        }
    }
}
