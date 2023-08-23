use crate::prelude::*;

pub fn tabs<'a>(
    selected: usize,
    tab: impl Fn(&str, bool) -> Element,
    tabs: impl IntoIterator<Item = &'a str>,
    view: impl Into<Element>,
) -> Element {
    linear(Axis::V)
        .child(
            linear(Axis::H)
                .spacing(1)
                .children(
                    tabs.into_iter()
                        .enumerate()
                        .map(|(i, name)| tab(name, i == selected)),
                )
                .width(Max),
        )
        .child(view)
        .into()
}

pub fn tab<'a>(name: &'a str, selected: bool) -> Element {
    let len = 14;
    let name = if name.len() > len {
        format!("[{}...]", &name[..len - 3])
    } else {
        format!("[{:len$}]", &name)
    };
    text(name)
        .apply_if(selected, |txt| {
            txt.style(Style::new().fg(Color::BLACK).bg(Color::WHITE))
        })
        .width(len + 2)
        .into()
}
