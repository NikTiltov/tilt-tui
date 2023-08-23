use crate::{prelude::*, Widget};

pub struct InfoBox {
    inner: Element,
}

impl InfoBox {
    pub fn new(widget: impl Into<Element>) -> Self {
        Self {
            inner: widget.into(),
        }
    }
}

impl Widget for InfoBox {
    fn size(&self) -> Size<Len> {
        self.inner.size()
    }

    fn layout(&self, limits: Limits) -> Layout {
        self.inner.layout(limits)
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        self.inner.render(layout, canvas);
        let Rect { x, y, w, h } = layout.rect();
        canvas.draw_str(x, y, &format!("[{}:{} {}:{}]", x, y, w, h));
    }
}

pub trait InfoExt: Into<Element> {
    fn show_info(self) -> InfoBox {
        InfoBox::new(self)
    }
}

impl<T: Into<Element>> InfoExt for T {}
