use crate::{prelude::*, Widget};

pub struct BorderBox {
    inner: Element,
    borders: Borders,
}

impl BorderBox {
    pub fn new(widget: impl Into<Element>, borders: Borders) -> Self {
        Self {
            inner: widget.into(),
            borders,
        }
    }
}

impl Widget for BorderBox {
    fn size(&self) -> Size<Len> {
        self.inner.size()
    }

    fn layout(&self, limits: Limits) -> Layout {
        let max = Size::new(
            limits.max.w.saturating_sub(2),
            limits.max.h.saturating_sub(2),
        );
        let min = Size::new(
            limits.min.w.saturating_sub(2),
            limits.min.h.saturating_sub(2),
        );
        let mut layout = self.inner.layout(Limits::new(min, max));
        layout.move_to(1, 1);
        let (w, h) = layout.size().into();
        let size = Size::new(w + 2, h + 2);
        Layout::new(limits.clamp(size)).with_child(layout)
    }

    fn render(&self, layout: &Layout, renderer: &mut Renderer) {
        let rect = layout.rect();
        let (x_, _x) = (rect.x, rect.end_x());
        let (y_, _y) = (rect.y, rect.end_y());
        if rect.size().area() != 0 {
            for x in x_.._x {
                renderer.cell_mut(x, y_).ch = self.borders.top;
                renderer.cell_mut(x, _y - 1).ch = self.borders.bottom;
            }
            for y in y_.._y {
                renderer.cell_mut(x_, y).ch = self.borders.left;
                renderer.cell_mut(_x - 1, y).ch = self.borders.right;
            }
            renderer.cell_mut(x_, y_).ch = self.borders.upper_left;
            renderer.cell_mut(x_, _y - 1).ch = self.borders.lower_left;
            renderer.cell_mut(_x - 1, y_).ch = self.borders.upper_right;
            renderer.cell_mut(_x - 1, _y - 1).ch = self.borders.lower_right;
        }
        self.inner.render(layout.first_child(), renderer);
    }
}

pub trait BorderExt: Into<Element> {
    fn borders(self, borders: Borders) -> BorderBox {
        BorderBox::new(self, borders)
    }

    fn case(self) -> BorderBox {
        BorderBox::new(self, Borders::NORMAL)
    }
}

impl<T: Into<Element>> BorderExt for T {}
