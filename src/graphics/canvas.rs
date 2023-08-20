use super::{Cell, Size};

pub struct Canvas {
    content: Vec<Cell>,
    size: Size,
    cell: Cell,
}

impl Canvas {
    pub fn empty(size: Size) -> Self {
        Self::filled(size, Cell::default())
    }

    pub fn filled(size: Size, cell: Cell) -> Self {
        let area = size.area();
        let mut content = Vec::with_capacity(area);
        content.resize(area, cell);
        let cell = Cell::default();
        Self {
            content,
            size,
            cell,
        }
    }

    pub fn content<'a>(&'a self) -> Vec<(usize, usize, &'a Cell)> {
        let mut content = Vec::new();
        for (i, cell) in self.content.iter().enumerate() {
            let x = i % self.size.w;
            let y = i / self.size.w;
            content.push((x, y, cell));
        }
        content
    }

    pub fn clear(&mut self) {
        self.content.fill(Cell::default());
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        if x < self.size.w && y < self.size.h {
            &mut self.content[self.size.w * y + x]
        } else {
            &mut self.cell
        }
    }

    pub fn draw_str(&mut self, x: usize, y: usize, str: &str) {
        for (i, char) in str.chars().enumerate() {
            self.cell_mut(x + i, y).symbol = char;
        }
    }
}
