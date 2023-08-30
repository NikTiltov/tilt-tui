use crate::{
    core::Element,
    events::{Event, KeyEvent},
    graphics::Limits,
    terminal::Terminal,
};

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
        let mut limits = Limits::from_max(terminal.size());

        let mut root = self.view();
        let mut layout = root.layout(limits);

        let mut update = true;
        loop {
            if update {
                let renderer = terminal.renderer();
                root.render(&layout, renderer);
                terminal.flush();
                update = false;
            }

            match terminal.event() {
                Event::KeyEvent(event) => match self.update(event) {
                    UpdateResult::Idle => {}
                    UpdateResult::Update => {
                        root = self.view();
                        layout = root.layout(limits);
                        update = true;
                    }
                    UpdateResult::Exit => break,
                },
                Event::Resize(size) => {
                    limits = Limits::from_max(size);
                    layout = root.layout(limits);
                    update = true;
                }
            }
        }
    }
}
