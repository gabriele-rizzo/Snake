use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

pub struct PauseMenu;

impl Widget for PauseMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = (18, 3);

        let mut area = area.inner(&Margin::new(
            (area.width - width) / 2,
            (area.height - height) / 2,
        ));

        let title = Title::from("| Pause Menu |".bold()).alignment(Alignment::Center);
        let block = Block::default().title(title).borders(Borders::ALL);

        block.render(area, buf);

        area = area.inner(&Margin::new(2, 1));
        Paragraph::new("R".bold()).render(Rect::new(area.x, area.y, 1, 1), buf);
        Paragraph::new(": Resume").render(Rect::new(area.x + 1, area.y, 13, 1), buf);

        area = area.inner(&Margin::new(0, 1));
        Paragraph::new("Q".bold()).render(Rect::new(area.x, area.y, 1, 1), buf);
        Paragraph::new(": Quit").render(Rect::new(area.x + 1, area.y, 13, 1), buf);
    }
}
