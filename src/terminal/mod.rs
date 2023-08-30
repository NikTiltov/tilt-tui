mod backend;
mod crossterm;

use backend::Backend;

use crate::{
    events::Event,
    graphics::{Cell, Renderer, Size},
};

pub struct Terminal {
    backend: Backend,
    renderers: [Renderer; 2],
    current: usize,
}

impl Terminal {
    pub fn new() -> Self {
        let backend = Backend::new();
        let renderers = Self::create_renderers(backend.size());
        Self {
            backend,
            renderers,
            current: 0,
        }
    }

    fn create_renderers(size: Size) -> [Renderer; 2] {
        [
            Renderer::empty(size),
            Renderer::filled(size, Cell::from('#')),
        ]
    }

    pub fn event(&mut self) -> Event {
        let event = self.backend.input();
        if let Event::Resize(size) = event {
            self.renderers = Self::create_renderers(size);
            self.current = 0;
        }
        event
    }

    pub fn size(&self) -> Size {
        self.backend.size()
    }

    pub fn flush(&mut self) {
        let diff = self.renderers[self.current]
            .diff(&self.renderers[1 - self.current]);
        self.backend.draw(diff);
        self.backend.flush();
        self.current = 1 - self.current;
        self.renderers[self.current].clear();
    }

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderers[self.current]
    }
}
