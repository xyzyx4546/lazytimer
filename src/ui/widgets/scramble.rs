use ratatui::{prelude::*, widgets::*};

pub struct ScrambleWidget {
    scramble: String,
}

impl ScrambleWidget {
    pub fn new(scramble: &String) -> ScrambleWidget {
        ScrambleWidget {
            scramble: scramble.to_string(),
        }
    }
}

impl Widget for ScrambleWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Scramble")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        Paragraph::new(self.scramble)
            .alignment(Alignment::Center)
            .style(Style::new().fg(Color::Magenta))
            .render(inner, buf);
    }
}

