use crate::prelude::*;

pub fn tabs<'a>(selected: usize) -> Tabs<'a> {
    Tabs::new(selected)
}

struct Tab<'a> {
    pub name: &'a str,
    pub view: Box<dyn Fn() -> Element + 'a>,
}

pub struct Tabs<'a> {
    selected: usize,
    tab_view: Box<dyn Fn(&str, bool) -> Element + 'a>,
    tabs: Vec<Tab<'a>>,
}

impl<'a> Tabs<'a> {
    pub fn new(selected: usize) -> Self {
        Self {
            selected,
            tab_view: Box::new(tab),
            tabs: Vec::new(),
        }
    }

    fn view(self) -> Element {
        linear(Axis::V)
            .child(
                linear(Axis::H)
                    .spacing(1)
                    .children(self.tabs.iter().enumerate().map(|(i, tab)| {
                        (self.tab_view)(tab.name, i == self.selected)
                    }))
                    .width(Max),
            )
            .child((self.tabs[self.selected].view)())
            .into()
    }

    pub fn tab_view(
        mut self,
        view: impl Fn(&str, bool) -> Element + 'a,
    ) -> Self {
        self.tab_view = Box::new(view);
        self
    }

    pub fn add_tab(
        mut self,
        name: &'a str,
        view: impl Fn() -> Element + 'a,
    ) -> Self {
        self.tabs.push(Tab {
            name,
            view: Box::new(view),
        });
        self
    }
}

impl<'a> From<Tabs<'a>> for Element {
    fn from(tabs: Tabs<'a>) -> Self {
        Element::from(tabs.view())
    }
}

pub fn tab<'a>(name: &'a str, selected: bool) -> Element {
    let len = 16;
    let name = if name.len() > len - 3 {
        format!("[{}...]", &name[..len - 3])
    } else {
        format!("[{:len$}]", &name)
    };
    text(name)
        .apply_if(selected, |txt| txt.mods(CellMods::Reverse))
        .width(len + 2)
        .into()
}
