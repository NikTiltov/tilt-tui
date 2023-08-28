use crate::{prelude::*, Widget};

pub fn canvas(draw_fn: impl Fn(Rect, &mut Renderer) + 'static) -> Canvas {
    Canvas::new(draw_fn)
}

pub struct Canvas {
    draw_fn: Box<dyn Fn(Rect, &mut Renderer)>,
}

impl Canvas {
    pub fn new(draw_fn: impl Fn(Rect, &mut Renderer) + 'static) -> Self {
        Self {
            draw_fn: Box::new(draw_fn),
        }
    }
}

impl Widget for Canvas {
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        Layout::new(limits.clamp(Size::new(0, 0)))
    }

    fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        (self.draw_fn)(layout.rect(), renderer);
    }
}
