use ratatui::{prelude::*, widgets::*};

use crate::{app::App, scramble::Scramble};

pub struct ScrambleWidget {
    scramble: Scramble,
}

impl ScrambleWidget {
    pub fn new(app: &App) -> ScrambleWidget {
        ScrambleWidget {
            scramble: app.current_scramble.clone(),
        }
    }
}

impl Widget for ScrambleWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Scramble")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        Paragraph::new(self.scramble.to_string())
            .centered()
            .style(Style::new().fg(Color::Magenta))
            .render(inner, buf);
    }
}
