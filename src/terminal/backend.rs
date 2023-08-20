use crate::{
    events::Event,
    graphics::{Cell, Size},
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, DisableMouseCapture},
    execute, queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
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
            DisableMouseCapture
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
        queue!(self.stdout, Hide).unwrap();
        for (x, y, cell) in content {
            queue!(
                self.stdout,
                MoveTo(x as u16, y as u16),
                SetForegroundColor(cell.fg.into()),
                SetBackgroundColor(cell.bg.into()),
                Print(cell.symbol),
            )
            .unwrap();
        }
        queue!(self.stdout, Show).unwrap();
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
