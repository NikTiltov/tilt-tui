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

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        let rect = layout.rect();
        let (x_, _x) = (rect.x, rect.end_x());
        let (y_, _y) = (rect.y, rect.end_y());
        if rect.size().area() != 0 {
            for x in x_.._x {
                canvas.cell_mut(x, y_).symbol = self.borders.top;
                canvas.cell_mut(x, _y - 1).symbol = self.borders.bottom;
            }
            for y in y_.._y {
                canvas.cell_mut(x_, y).symbol = self.borders.left;
                canvas.cell_mut(_x - 1, y).symbol = self.borders.right;
            }
            canvas.cell_mut(x_, y_).symbol = self.borders.upper_left;
            canvas.cell_mut(x_, _y - 1).symbol = self.borders.lower_left;
            canvas.cell_mut(_x - 1, y_).symbol = self.borders.upper_right;
            canvas.cell_mut(_x - 1, _y - 1).symbol = self.borders.lower_right;
        }
        self.inner.render(layout.first_child(), canvas);
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
