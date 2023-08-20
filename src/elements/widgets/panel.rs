use crate::{prelude::*, Widget};

pub fn panel(fill: char) -> Panel {
    Panel::new(fill)
}

pub struct Panel {
    fill: char,
    style: Style,
    size: Size<Length>,
}

impl Panel {
    pub fn new(fill: char) -> Self {
        Self {
            fill,
            style: Style::default(),
            size: Size::max(),
        }
    }

    pub fn size(
        mut self,
        width: impl Into<Length>,
        height: impl Into<Length>,
    ) -> Self {
        self.size = Size::new(width.into(), height.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Panel {
    fn size(&self) -> Size<Length> {
        self.size
    }

    fn layout(&self, bound: Size) -> Layout {
        let len = |axis| match self.size.main(axis) {
            Length::Var(var) => var,
            Length::Max => bound.main(axis),
            Length::Min => 0,
        };
        let size = Size::new(len(Axis::H), len(Axis::V));
        Layout::new(size)
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
