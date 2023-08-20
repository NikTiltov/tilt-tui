use crate::{
    core::Widget,
    graphics::{Canvas, Layout, Length, Size},
};

pub struct Element {
    widget: Box<dyn Widget>,
}

impl Element {
    pub fn size(&self) -> Size<Length> {
        self.widget.size()
    }

    pub fn layout(&self, bound: Size) -> Layout {
        self.widget.layout(bound)
    }

    pub fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        self.widget.render(layout, canvas);
    }
}

impl<'a, T: Widget + 'static> From<T> for Element {
    fn from(widget: T) -> Self {
        Element {
            widget: Box::new(widget),
        }
    }
}
