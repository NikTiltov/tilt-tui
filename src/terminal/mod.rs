mod backend;
mod crossterm;

use backend::Backend;

use crate::{
    events::Event,
    graphics::{Canvas, Size},
};

pub struct Terminal {
    backend: Backend,
    buffer: Canvas,
}

impl Terminal {
    pub fn new() -> Self {
        let backend = Backend::new();
        let buffer = Canvas::empty(backend.size());
        Self { backend, buffer }
    }

    pub fn event(&mut self) -> Event {
        let event = self.backend.input();
        if let Event::Resize(size) = event {
            self.buffer = Canvas::empty(size);
        }
        event
    }

    pub fn size(&self) -> Size {
        self.backend.size()
    }

    pub fn flush(&mut self) {
        self.backend.draw(self.buffer.content());
        self.backend.flush();
        self.buffer.clear();
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        &mut self.buffer
    }
}
