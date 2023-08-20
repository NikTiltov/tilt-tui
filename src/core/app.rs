use crate::{
    events::{Event, KeyEvent},
    terminal::Terminal,
};

use super::Element;

pub enum UpdateResult {
    Idle,
    Update,
    Exit,
}

pub trait App: Sized {
    fn view(&self) -> Element;

    fn update(&mut self, event: KeyEvent) -> UpdateResult;

    fn run(mut self) {
        let mut terminal = Terminal::new();

        let mut root = self.view();
        let mut layout = root.layout(terminal.size().into());

        let mut update = true;
        loop {
            if update {
                let canvas = terminal.canvas();
                root.render(&layout, canvas);
                terminal.flush();
                update = false;
            }

            match terminal.event() {
                Event::KeyEvent(event) => match self.update(event) {
                    UpdateResult::Idle => {}
                    UpdateResult::Update => {
                        root = self.view();
                        layout = root.layout(terminal.size().into());
                        update = true;
                    }
                    UpdateResult::Exit => break,
                },
                Event::Resize(size) => {
                    layout = root.layout(size.into());
                    update = true;
                }
            }
        }
    }
}
