use crate::{prelude::*, Widget};

pub trait Cased {
    fn cased(self) -> Case;
}

impl<'a, T: Into<Element>> Cased for T {
    fn cased(self) -> Case {
        Case::new(self)
    }
}

pub struct Case {
    widget: Element,
    borders: Option<Borders>,
    show_info: bool,
    style: Style,
}

impl Case {
    pub fn new(widget: impl Into<Element>) -> Self {
        Self {
            widget: widget.into(),
            borders: None,
            show_info: false,
            style: Style::new(),
        }
    }

    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    pub fn fg_color(mut self, color: Color) -> Self {
        self.style = self.style.fg(color);
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.style = self.style.bg(color);
        self
    }

    pub fn show_info(mut self) -> Self {
        self.show_info = true;
        self
    }
}

impl Widget for Case {
    fn size(&self) -> Size<Length> {
        self.widget.size()
    }

    fn layout(&self, bound: Size) -> Layout {
        let mut size = bound;
        if self.borders.is_some() {
            size.w = size.w.saturating_sub(2);
            size.h = size.h.saturating_sub(2);
        };
        let mut layout = self.widget.layout(size);
        let mut size = layout.size();
        if self.borders.is_some() {
            layout.move_to(1, 1);
            size.w = size.w.saturating_add(2);
            size.h = size.h.saturating_add(2);
        };
        Layout::new(size).with_child(layout)
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        let rect = layout.rect();
        let (x_, y_) = (rect.x, rect.y);
        let (_x, _y) = (rect.end_x(), rect.end_y());
        for x in x_.._x {
            for y in y_.._y {
                let cell = canvas.cell_mut(x, y);
                cell.set_symbol(' ');
                cell.set_style(self.style);
            }
        }
        if let Some(ref borders) = self.borders {
            for x in x_.._x {
                canvas.cell_mut(x, y_).symbol = borders.top;
                canvas.cell_mut(x, _y - 1).symbol = borders.bottom;
            }
            for y in y_.._y {
                canvas.cell_mut(x_, y).symbol = borders.left;
                canvas.cell_mut(_x - 1, y).symbol = borders.right;
            }
            canvas.cell_mut(x_, y_).symbol = borders.upper_left;
            canvas.cell_mut(x_, _y - 1).symbol = borders.lower_left;
            canvas.cell_mut(_x - 1, y_).symbol = borders.upper_right;
            canvas.cell_mut(_x - 1, _y - 1).symbol = borders.lower_right;
        }

        if self.show_info {
            let Rect { x, y, w, h } = layout.rect();
            let string = format!("{}:{}-{}:{}", x, y, w, h);
            canvas.draw_str(x_ + 1, y_, &string);
        }
        let layout = &layout.children()[0];
        self.widget.render(layout, canvas);
    }
}
