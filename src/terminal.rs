use crossterm::{terminal::*, ExecutableCommand};
use ratatui::{backend::CrosstermBackend, Terminal as RatatuiTerminal};
use std::io;

pub struct Terminal {
    backend: RatatuiTerminal<CrosstermBackend<io::Stdout>>,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let backend = RatatuiTerminal::new(CrosstermBackend::new(io::stdout()))?;

        crossterm::terminal::enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;

        Ok(Self { backend })
    }

    pub fn draw(&mut self, f: impl FnOnce(&mut ratatui::Frame)) -> io::Result<()> {
        self.backend.draw(f).map(|_| ())
    }

    pub fn size() -> (u16, u16) {
        crossterm::terminal::size().unwrap()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
        io::stdout().execute(LeaveAlternateScreen).unwrap();
    }
}
