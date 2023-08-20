use tilt_tui::prelude::*;

fn main() {
    Counter::new().run();
}

struct Counter {
    ratio: f64,
    marked: bool,
    tab_id: usize,
    key_event: KeyEvent,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            marked: false,
            ratio: 0.0,
            tab_id: 0,
            key_event: KeyEvent::code(KeyCode::ENTER),
        }
    }
}

static TEXT: &str = "The simple text of the text, with text.
The simple text of the text, with text. Apple of company maceboke.";

impl App for Counter {
    fn view(&self) -> Element {
        let list = {
            let item = |name, marked| {
                linear(Axis::H)
                    .child(text(name).size(10, Min))
                    .child(toggle(marked))
            };
            linear(Axis::V)
                .child(item("item 1", true))
                .child(item("item 2", self.marked))
                .child(item("item 3", false))
                .cased()
                .borders(Borders::NORMAL)
        };

        let align_text = || {
            let text_box = |x, y| {
                let h = match x {
                    0 => Alignment::Start,
                    1 => Alignment::Center,
                    _ => Alignment::End,
                };
                let v = match y {
                    0 => Alignment::Start,
                    1 => Alignment::Center,
                    _ => Alignment::End,
                };
                text("qwe\nrtyui")
                    .size(Max, Max)
                    .align_h(h)
                    .align_v(v)
                    .cased()
                    .borders(Borders::NORMAL)
            };

            let row = move |y| {
                linear(Axis::H)
                    .size(Max, Max)
                    .child(text_box(0, y))
                    .child(text_box(1, y))
                    .child(text_box(2, y))
                    .spacing(1)
            };

            linear(Axis::V)
                .size(Max, Max)
                .child(row(0))
                .child(row(1))
                .child(row(2))
                .into()
        };

        let progress_bar = range(self.ratio)
            .size(Max, 2)
            .cased()
            .borders(Borders::NORMAL);

        let tab_view = |txt| {
            text(txt)
                .size(Max, Max)
                .cased()
                .borders(Borders::NORMAL)
                .into()
        };

        let tabs = {
            let view = match self.tab_id {
                0 => align_text(),
                1 => tab_view("2"),
                _ => tab_view("3"),
            };
            tabs(self.tab_id, tab, ["Text alignment", "Tab 2", "Tab 3"], view)
                .cased()
                .borders(Borders::NORMAL)
        };

        linear(Axis::V)
            .size(Max, Max)
            .child(
                text(format!("{:?}", self.key_event))
                    .cased()
                    .borders(Borders::NORMAL),
            )
            .child(text(TEXT).cased().borders(Borders::NORMAL))
            .child(list)
            .child(progress_bar)
            .child(tabs)
            .cased()
            .borders(Borders::NORMAL)
            .into()
    }

    fn update(&mut self, event: KeyEvent) -> UpdateResult {
        match event.code {
            KeyCode::ESC => return UpdateResult::Exit,
            KeyCode::ENTER => {
                self.marked = !self.marked;
            }
            KeyCode::RIGHT => {
                self.ratio = (self.ratio + 0.01).min(1.0);
            }
            KeyCode::LEFT => {
                self.ratio = (self.ratio - 0.01).max(0.0);
            }
            KeyCode::TAB => {
                self.tab_id = (self.tab_id + 1) % 3;
            }
            KeyCode::BACKTAB => {
                self.tab_id = self.tab_id.wrapping_sub(1).min(2);
            }
            _ => {}
        }
        self.key_event = event;
        UpdateResult::Update
    }
}
