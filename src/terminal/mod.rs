mod backend;
mod crossterm;

use backend::Backend;

use crate::{
    events::Event,
    graphics::{Renderer, Size},
};

pub struct Terminal {
    backend: Backend,
    renderer: Renderer,
}

impl Terminal {
    pub fn new() -> Self {
        let backend = Backend::new();
        let renderer = Renderer::empty(backend.size());
        Self { backend, renderer }
    }

    pub fn event(&mut self) -> Event {
        let event = self.backend.input();
        if let Event::Resize(size) = event {
            self.renderer = Renderer::empty(size);
        }
        event
    }

    pub fn size(&self) -> Size {
        self.backend.size()
    }

    pub fn flush(&mut self) {
        self.backend.draw(self.renderer.content());
        self.backend.flush();
        self.renderer.clear();
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
