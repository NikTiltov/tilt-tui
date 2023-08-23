use crate::graphics::{Canvas, Layout, Len, Limits, Size};

pub trait Widget {
    fn size(&self) -> Size<Len>;
    fn layout(&self, limits: Limits) -> Layout;
    fn render(&self, layout: &Layout, canvas: &mut Canvas);
}
