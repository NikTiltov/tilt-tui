use tilt_tui::prelude::*;

pub struct Doc {
    id: usize,
    name: String,
}

impl Doc {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn view<'a>(&self) -> Element {
        text("txt").into()
    }

    pub fn update(&mut self) -> UpdateResult {
        UpdateResult::Idle
    }
}
