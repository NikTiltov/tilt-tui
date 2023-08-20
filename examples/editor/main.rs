use tilt_tui::prelude::*;

mod doc;
mod files;
mod tab;
mod theme;

use tab::Tab;

fn main() {
    Editor::new().run();
}

struct Editor {
    tabs: Vec<Tab>,
    tab_id: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::new_doc("main.rs")],
            tab_id: 0,
        }
    }

    pub fn cur_tab(&self) -> &Tab {
        &self.tabs[self.tab_id]
    }

    pub fn cur_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.tab_id]
    }
}

impl App for Editor {
    fn view(&self) -> Element {
        let tabs = {
            tabs(
                self.tab_id,
                tab,
                self.tabs.iter().map(|tab| tab.name()),
                self.cur_tab().view(),
            )
        };

        linear(Axis::V)
            .size(Max, Max)
            .child(tabs)
            .cased()
            .fg_color(theme::pallete::N1)
            .bg_color(theme::pallete::D0)
            .into()
    }

    fn update(&mut self, event: KeyEvent) -> UpdateResult {
        match event.code {
            KeyCode::ESC => UpdateResult::Exit,
            KeyCode::LEFT if event.mods == KeyMods::SHIFT => {
                if self.tab_id == 0 {
                    self.tab_id = self.tabs.len();
                }
                self.tab_id -= 1;
                UpdateResult::Update
            }
            KeyCode::RIGHT if event.mods == KeyMods::SHIFT => {
                self.tab_id += 1;
                if self.tab_id == self.tabs.len() {
                    self.tab_id = 0;
                }
                UpdateResult::Update
            }
            _ => UpdateResult::Idle,
        }
    }
}

pub fn tab<'a>(name: &'a str, selected: bool) -> Element {
    let len = 14;
    let name = if name.len() > len {
        format!("[{}...]", &name[..len - 3])
    } else {
        format!("[{:len$}]", &name)
    };
    let (fg, bg) = if selected {
        (theme::pallete::D0, theme::pallete::N1)
    } else {
        (theme::pallete::N1, theme::pallete::D0)
    };
    text(name)
        .size(len + 2, Min)
        .style(Style::new().fg(fg).bg(bg))
        .into()
}
