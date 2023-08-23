use crate::{prelude::*, Widget};

pub fn panel(fill: char) -> Panel {
    Panel::new(fill)
}

pub struct Panel {
    fill: char,
    style: Style,
}

impl Panel {
    pub fn new(fill: char) -> Self {
        Self {
            fill,
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Panel {
    fn size(&self) -> Size<Len> {
        Size::min()
    }

    fn layout(&self, limits: Limits) -> Layout {
        Layout::new(limits.clamp(Size::new(0, 0)))
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        let rect = layout.rect();
        for y in rect.y..rect.end_y() {
            for x in rect.x..rect.end_x() {
                let cell = canvas.cell_mut(x, y);
                cell.set_symbol(self.fill);
                cell.set_style(self.style);
            }
        }
    }
}
