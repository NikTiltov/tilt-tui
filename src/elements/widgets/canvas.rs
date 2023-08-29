use crate::{prelude::*, Widget};

pub fn canvas<'a>(draw_fn: impl Fn(Rect, &mut Renderer) + 'a) -> Canvas<'a> {
    Canvas::new(draw_fn)
}

pub struct Canvas<'a> {
    draw_fn: Box<dyn Fn(Rect, &mut Renderer) + 'a>,
}

impl<'a> Canvas<'a> {
    pub fn new(draw_fn: impl Fn(Rect, &mut Renderer) + 'a) -> Self {
        Self {
            draw_fn: Box::new(draw_fn),
        }
    }
}

impl<'a> Widget for Canvas<'a> {
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
