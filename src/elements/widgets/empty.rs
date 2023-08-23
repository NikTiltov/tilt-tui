use crate::{prelude::*, Widget};

pub fn empty() -> Empty {
    Empty
}

pub struct Empty;

impl Widget for Empty {
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        Layout::new(limits.clamp(Size::new(0, 0)))
    }

    fn render(&self, _layout: &Layout, _canvas: &mut Canvas) {}
}
