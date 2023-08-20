use crate::{prelude::*, Widget};

pub fn empty() -> Empty {
    Empty
}

pub struct Empty;

impl Widget for Empty {
    fn size(&self) -> Size<Length> {
        Size::min()
    }

    fn layout(&self, _bound: Size) -> Layout {
        Layout::new(Size::new(0, 0))
    }

    fn render(&self, _layout: &Layout, _canvas: &mut Canvas) {}
}
