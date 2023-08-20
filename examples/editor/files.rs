use super::theme::*;
use tilt_tui::prelude::*;

pub fn open(file: Vec<Vec<Span<'static>>>) -> Element {
    let lines: Vec<Line<'static>> = file
        .into_iter()
        .enumerate()
        .map(|(i, mut l)| {
            l.insert(0, span(format!("{:>3} ", i)).fg(OTH));
            line(l)
        })
        .collect();
    text(lines).size(Max, Max).into()
}

pub fn main_rs<'a>() -> Vec<Vec<Span<'a>>> {
    vec![
        vec![
            span("fn ").fg(KEY),
            span("main").fg(FUN),
            span("() {").fg(DEF),
        ],
        vec![
            span("    let ").fg(KEY),
            span("a ").fg(VAR),
            span("= ").fg(DEF),
            span("10usize").fg(NUM),
            span(";").fg(DEF),
        ],
        vec![
            span("    let ").fg(KEY),
            span("b ").fg(VAR),
            span("= ").fg(DEF),
            span("20usize").fg(NUM),
            span(";").fg(DEF),
        ],
        vec![
            span("    println!").fg(FUN),
            span("(").fg(DEF),
            span("\"{}\"").fg(STR),
            span(", ").fg(DEF),
            span("a ").fg(VAR),
            span("+ ").fg(DEF),
            span("b").fg(VAR),
            span(");").fg(DEF),
        ],
        vec![span("}").fg(DEF)],
    ]
}
