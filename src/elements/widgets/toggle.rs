use crate::{prelude::*, Widget};

pub fn toggle<'a>(marked: bool) -> Toggle<'a> {
    Toggle::new(marked)
}

pub struct Toggle<'a> {
    marked: bool,
    marked_view: &'a str,
    unmarked_view: &'a str,
}

impl<'a> Toggle<'a> {
    pub fn new(marked: bool) -> Self {
        Self {
            marked,
            marked_view: "[X]",
            unmarked_view: "[ ]",
        }
    }

    fn current_view(&self) -> &str {
        if self.marked {
            self.marked_view
        } else {
            self.unmarked_view
        }
    }
}

impl<'a> Widget for Toggle<'a> {
    fn size(&self) -> Size<Length> {
        Size::min()
    }

    fn layout(&self, _bound: Size) -> Layout {
        Layout::new(Size::new(self.current_view().len(), 1))
    }

    fn render(&self, layout: &Layout, canvas: &mut Canvas) {
        let rect = layout.rect();
        for (i, char) in self.current_view().chars().enumerate() {
            canvas.cell_mut(rect.x + i, rect.y).symbol = char;
        }
    }
}
