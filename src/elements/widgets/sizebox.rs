use crate::{prelude::*, Widget};

pub struct Sizebox<'a> {
    inner: Element<'a>,
    width: Length,
    height: Length,
}

impl<'a> Sizebox<'a> {
    pub fn new(widget: impl Into<Element<'a>>) -> Self {
        Self {
            inner: widget.into(),
            width: Length::Min,
            height: Length::Min,
        }
    }

    pub fn set_width(&mut self, value: impl Into<Length>) {
        self.width = value.into();
    }

    pub fn with_width(mut self, value: impl Into<Length>) -> Self {
        self.set_width(value);
        self
    }

    pub fn set_height(&mut self, value: impl Into<Length>) {
        self.height = value.into();
    }

    pub fn with_height(mut self, value: impl Into<Length>) -> Self {
        self.set_height(value);
        self
    }
}

impl<'a> Widget for Sizebox<'a> {
    fn size(&self) -> Size<Length> {
        todo!()
    }

    fn layout(&self, bound: Size) -> Layout {
        self.inner.layout(bound)
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        self.inner.render(layout, canvas);
    }
}
