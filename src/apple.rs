use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};

use crate::terminal::Terminal;

#[derive(Clone, Copy)]
pub struct Apple {
    pub position: (u16, u16),
}

impl Default for Apple {
    fn default() -> Self {
        let mut generator = rand::thread_rng();
        let area = Terminal::size();

        let x = generator.gen_range(1..area.0 - 1);
        let y = generator.gen_range(1..area.1 - 1);

        Self { position: (x, y) }
    }
}

impl Widget for Apple {
    fn render(self, _: Rect, buf: &mut Buffer) {
        Paragraph::new("◉".bold()).render(Rect::new(self.position.0, self.position.1, 1, 1), buf);
    }
}
