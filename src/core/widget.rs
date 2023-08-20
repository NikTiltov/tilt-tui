use crate::graphics::{Canvas, Layout, Length, Size};

pub trait Widget {
    fn size(&self) -> Size<Length>;
    fn layout(&self, bound: Size) -> Layout;
    fn render(&self, layout: &Layout, canvas: &mut Canvas);
}
