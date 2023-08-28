use crate::{prelude::*, Widget};

pub fn panel(fill: char) -> Panel {
    Panel::new(fill)
}

pub struct Panel {
    fill: char,
}

impl Panel {
    pub fn new(fill: char) -> Self {
        Self { fill }
    }
}

impl Widget for Panel {
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        Layout::new(limits.clamp(Size::new(0, 0)))
    }

    fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        let rect = layout.rect();
        for y in rect.y..rect.end_y() {
            for x in rect.x..rect.end_x() {
                renderer.cell_mut(x, y).set_symbol(self.fill);
            }
        }
    }
}
