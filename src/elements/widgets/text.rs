use crate::{prelude::*, Widget};

pub fn text<'a>(txt: impl Into<Text<'a>>) -> Textbox<'a> {
    Textbox::new(txt)
}

pub struct Textbox<'a> {
    content: Text<'a>,
    align_h: Alignment,
    align_v: Alignment,
    wrap: Wrap,
}

impl<'a> Textbox<'a> {
    pub fn new(txt: impl Into<Text<'a>>) -> Self {
        Self {
            content: txt.into(),
            align_h: Alignment::Start,
            align_v: Alignment::Start,
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
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        let text = self.wrap.apply(&self.content, limits.max.w);
        let width = text.iter().map(|line| line.len()).max().unwrap();
        let height = text.len();
        Layout::new(limits.clamp(Size::new(width, height)))
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
