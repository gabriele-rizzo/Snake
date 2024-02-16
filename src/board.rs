use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

pub struct Board {
    pub score: u16,
    pub best: Option<u16>,
}

impl Widget for Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("| Snake |".bold()).alignment(Alignment::Center);
        let score = Title::from(format!("| Score: {} |", self.score)).position(Position::Bottom);

        let mut block = Block::default().title(title).title(score);

        if let Some(best) = self.best {
            let best = Title::from(format!("| Best: {} |", best))
                .alignment(Alignment::Right)
                .position(Position::Bottom);

            block = block.title(best);
        }

        block.borders(Borders::ALL).render(area, buf);
    }
}
