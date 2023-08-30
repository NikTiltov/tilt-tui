use crate::{
    events::Event,
    graphics::{Cell, Size},
};
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, DisableMouseCapture},
    execute, queue,
    style::{
        Attribute, Print, SetAttribute, SetAttributes, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{stdout, Stdout, Write};

pub struct Backend {
    stdout: Stdout,
}

impl Backend {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            Clear(ClearType::All),
            DisableMouseCapture,
            Hide,
        )
        .unwrap();
        terminal::enable_raw_mode().unwrap();
        Self { stdout }
    }

    pub fn size(&self) -> Size {
        let (width, height) = terminal::size().unwrap();
        Size::new(width as usize, height as usize)
    }

    pub fn input(&mut self) -> Event {
        loop {
            if let Ok(event) = event::read() {
                if let Ok(event) = event.try_into() {
                    return event;
                }
            }
        }
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn draw<'a, T>(&mut self, content: T)
    where
        T: IntoIterator<Item = (usize, usize, &'a Cell)>,
    {
        for (x, y, cell) in content {
            queue!(
                self.stdout,
                MoveTo(x as u16, y as u16),
                SetAttributes(cell.mods.into()),
                SetForegroundColor(cell.fg.into()),
                SetBackgroundColor(cell.bg.into()),
                Print(cell.ch),
                SetAttribute(Attribute::Reset),
            )
            .unwrap();
        }
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
