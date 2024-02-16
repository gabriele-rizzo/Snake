use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

pub struct ResultMenu {
    pub state: bool,
}

impl Widget for ResultMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let string = if self.state { "You won!" } else { "You lost!" };
        let (width, height) = (18, 7);

        let mut area = area.inner(&Margin::new(
            (area.width - width) / 2,
            (area.height - height) / 2,
        ));

        area.width = width;
        area.height = height;

        let title = Title::from("| Result |").alignment(Alignment::Center);
        let block = Block::default().title(title).borders(Borders::ALL);

        block.render(area, buf);

        let length = string.len() as u16;
        area.y += 2;

        Paragraph::new(string.bold()).render(
            Rect::new(area.x + (area.width - length) / 2, area.y, length, 1),
            buf,
        );

        area = area.inner(&Margin::new(2, 2));
        Paragraph::new("R".bold()).render(Rect::new(area.x, area.y, 1, 1), buf);
        Paragraph::new(": Restart").render(Rect::new(area.x + 1, area.y, 13, 1), buf);

        area = area.inner(&Margin::new(0, 1));
        Paragraph::new("Q".bold()).render(Rect::new(area.x, area.y, 1, 1), buf);
        Paragraph::new(": Quit").render(Rect::new(area.x + 1, area.y, 13, 1), buf);
    }
}
