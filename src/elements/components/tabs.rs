use crate::prelude::*;

// pub fn tabs<'a>(
//     selected: usize,
//     tab: impl Fn(&'a str, bool) -> Element,
//     content: &[(&'a str, Box<dyn Fn() -> Element>)],
// ) -> Element {
//     linear(Axis::V)
//         .size(Max, Max)
//         .child(
//             linear(Axis::H).size(Max, Min).spacing(1).children(
//                 content
//                     .iter()
//                     .enumerate()
//                     .map(|(i, t)| tab(t.0, i == selected)),
//             ),
//         )
//         .child(content[selected].1())
//         .into()
// }

pub fn tabs<'a>(
    selected: usize,
    tab: impl Fn(&str, bool) -> Element,
    tabs: impl IntoIterator<Item = &'a str>,
    view: Element,
) -> Element {
    linear(Axis::V)
        .size(Max, Max)
        .child(
            linear(Axis::H).size(Max, Min).spacing(1).children(
                tabs.into_iter()
                    .enumerate()
                    .map(|(i, name)| tab(name, i == selected)),
            ),
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
    let mut text = text(name).size(len + 2, Min);
    if selected {
        text = text.style(Style {
            fg: Some(Color::BLACK),
            bg: Some(Color::WHITE),
        });
    }
    text.into()
}
