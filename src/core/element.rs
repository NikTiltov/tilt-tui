use crate::{
    core::Widget,
    graphics::{Canvas, Layout, Len, Limits, Size},
};

pub struct Element {
    widget: Box<dyn Widget>,
}

impl Element {
    pub fn size(&self) -> Size<Len> {
        self.widget.size()
    }

    pub fn layout(&self, limits: Limits) -> Layout {
        self.widget.layout(limits)
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
