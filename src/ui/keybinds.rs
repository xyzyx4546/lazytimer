use ratatui::{prelude::*, widgets::*};

pub struct Popup {}

impl Popup {
    pub fn new() -> Self {
        Self {}
    }
}

fn line<'a>(key: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{:>8}", key), Style::default().fg(Color::Magenta)),
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
            Line::raw(""),
            line("?", "Show keybinds"),
            line("q", "Quit"),
            line("Esc", "Close popup"),
            line("Enter", "Confirm"),
            line("Space", "Start/stop timer"),
            Line::raw(""),
            line("h / ←", "Previous session"),
            line("j / ↓", "Previous solve"),
            line("k / ↑", "Next solve"),
            line("l / →", "Next session"),
            line("g", "Go to first solve"),
            line("G", "Go to last solve"),
            Line::raw(""),
            line("i", "Show solve details"),
            line("+", "Toggle +2 penalty"),
            line("-", "Toggle DNF penalty"),
            line("d", "Delete selected solve"),
            line("D", "Delete selected session"),
            line("n", "Create new session"),
        ];

        Paragraph::new(text).block(block).render(area, buf);
    }
}
