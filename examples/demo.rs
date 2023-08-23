use tilt_tui::prelude::*;

fn main() {
    let _ = simple_logging::log_to_file("log.log", log::LevelFilter::Debug);
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        log::error!("{}", info);
    }));
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
        let key_event = text(format!("{:?}", self.key_event)).case();

        let text_box = text(TEXT).case();

        let list = {
            let item = |name, marked| {
                linear(Axis::H)
                    .child(text(name).width(10))
                    .child(toggle(marked))
            };
            linear(Axis::V)
                .child(item("item 1", true))
                .child(item("item 2", self.marked))
                .child(item("item 3", false))
                .case()
        };

        let progress_bar = range(self.ratio).width(Max).height(2).case();

        let tabs = {
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
                        .align_h(h)
                        .align_v(v)
                        .width(Max)
                        .height(Max)
                        .case()
                };

                let row = move |y| {
                    linear(Axis::H)
                        .child(text_box(0, y))
                        .child(text_box(1, y))
                        .child(text_box(2, y))
                        .spacing(1)
                        .width(Max)
                        .height(Max)
                };

                linear(Axis::V)
                    .child(row(0))
                    .child(row(1))
                    .child(row(2))
                    .width(Max)
                    .height(Max)
                    .into()
            };

            let split_view = || {
                let text_view = |name| {
                    text(name)
                        .align_h(Alignment::Center)
                        .align_v(Alignment::Center)
                        .width(Max)
                        .height(Max)
                };
                split(
                    Axis::H,
                    text_view("left"),
                    split(
                        Axis::V,
                        text_view("up"),
                        split(
                            Axis::H,
                            split(
                                Axis::V,
                                text_view("center"),
                                text_view("bottom"),
                            ),
                            text_view("right"),
                        ),
                    ),
                )
                .case()
                .width(Max)
                .height(Max)
                .into()
            };

            let tab_view = || text("tab").width(Max).height(Max).case().into();

            let view: Element = match self.tab_id {
                0 => align_text(),
                1 => split_view(),
                _ => tab_view(),
            };
            tabs(
                self.tab_id,
                tab,
                ["Text alignment", "Split view", "Tab 3"],
                view,
            )
            .borders(Borders::BOLD)
            .width(Max)
            .height(Max)
        };

        linear(Axis::V)
            .child(key_event)
            .child(text_box)
            .child(list)
            .child(progress_bar)
            .child(tabs)
            .width(Max)
            .height(Max)
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
