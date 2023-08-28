use crate::{
    core::Widget,
    graphics::{Layout, Len, Limits, Renderer, Size},
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

    pub fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        self.widget.render(layout, renderer);
    }
}

impl<'a, T: Widget + 'static> From<T> for Element {
    fn from(widget: T) -> Self {
        Element {
            widget: Box::new(widget),
        }
    }
}
