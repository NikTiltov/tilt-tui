use super::doc::Doc;
use tilt_tui::prelude::*;

pub struct Tab {
    name: String,
    view: Box<dyn Fn() -> Element>,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            name: "new tab".to_string(),
            view: Box::new(|| empty().into()),
        }
    }

    pub fn new_doc(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            view: Box::new(|| text("zxc").into()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn view(&self) -> Element {
        (self.view)()
    }
}
