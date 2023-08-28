use crate::graphics::{Layout, Len, Limits, Renderer, Size};

pub trait Widget {
    fn size(&self) -> Size<Len>;
    fn layout(&self, limits: Limits) -> Layout;
    fn render(&self, layout: &Layout, renderer: &mut Renderer);
}
