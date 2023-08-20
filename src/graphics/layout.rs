use crate::graphics::{Rect, Size};

#[derive(Debug, Default)]
pub struct Layout {
    rect: Rect,
    children: Vec<Layout>,
}

impl Layout {
    pub fn new(size: Size) -> Self {
        Self {
            rect: Rect::from(size),
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: Layout) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<Layout>) -> Self {
        self.children = children;
        self
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn size(&self) -> Size {
        self.rect.size()
    }

    pub fn children(&self) -> &[Layout] {
        &self.children
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.rect.x += x;
        self.rect.y += y;
        for child in self.children.iter_mut() {
            child.move_to(x, y);
        }
    }
}
