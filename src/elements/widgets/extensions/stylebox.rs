use crate::{prelude::*, Widget};

pub struct StyleBox {
    inner: Element,
    style: Style,
}

impl StyleBox {
    pub fn new(widget: impl Into<Element>) -> Self {
        Self {
            inner: widget.into(),
            style: Style::new(),
        }
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.style.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.style.bg = Some(color);
        self
    }
}

impl Widget for StyleBox {
    fn size(&self) -> Size<Len> {
        self.inner.size()
    }

    fn layout(&self, limits: Limits) -> Layout {
        self.inner.layout(limits)
    }

    fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        let rect = layout.rect();
        for x in rect.x..rect.end_x() {
            for y in rect.y..rect.end_y() {
                renderer.cell_mut(x, y).set_style(self.style);
            }
        }
        self.inner.render(layout, renderer);
    }
}

pub trait StyleExt: Into<Element> {
    fn fg(self, color: Color) -> StyleBox {
        StyleBox::new(self).fg(color)
    }

    fn bg(self, color: Color) -> StyleBox {
        StyleBox::new(self).bg(color)
    }
}

impl<T: Into<Element>> StyleExt for T {}
