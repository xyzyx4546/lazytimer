use ratatui::{prelude::*, widgets::*};

pub struct Popup {}

impl Popup {
    pub fn new() -> Self {
        Self {}
    }
}

fn line<'a>(key: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{:>10}", key), Style::default().fg(Color::Cyan)),
        Span::raw(format!("  {}", value)),
    ])
}

impl Widget for Popup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Keybinds")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = vec![
            line("?", "Show keybinds"),
            line("Esc", "Go back"),
            line("Space", "Start timer"),
            line("d", "Delete selected solve"),
            line("q", "Quit"),
        ];

        Paragraph::new(text).block(block).render(area, buf);
    }
}
