use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub struct StatisticsScreen<'a> {
    _app: &'a App,
}

impl<'a> StatisticsScreen<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { _app: app }
    }
}

impl<'a> Widget for StatisticsScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new("WIP")
            .block(
                Block::default()
                    .title("Statistics")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .centered();
        paragraph.render(area, buf);
    }
}
