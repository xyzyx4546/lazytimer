use ratatui::{prelude::*, widgets::*};
use std::time::{SystemTime, UNIX_EPOCH};
use tui_widgets::big_text::*;

use crate::{app::App, sessions::Penalty};

pub struct Popup<'a> {
    app: &'a App,
}

impl<'a> Popup<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

fn line(key: &str, value: String, color: Color) -> Line {
    Line::from(vec![
        Span::styled(format!("{:>10}  ", key), Style::default().fg(Color::Cyan)),
        Span::styled(value, Style::default().fg(color)),
    ])
}

fn format_date(timestamp: SystemTime) -> String {
    timestamp
        .duration_since(UNIX_EPOCH)
        .map(|d| {
            let secs = d.as_secs();
            let days = secs / 86400;
            let year = 1970 + days / 365;
            let month = ((days % 365) / 30) + 1;
            let day = ((days % 365) % 30) + 1;
            let time = secs % 86400;
            let (h, m, s) = (time / 3600, (time % 3600) / 60, time % 60);
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                year, month, day, h, m, s
            )
        })
        .unwrap_or_else(|_| "Invalid".to_string())
}

impl<'a> Widget for Popup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(format!("Solve #{}", self.app.selected_solve_idx + 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        let solve = self.app.selected_solve().unwrap();

        let ao_str = |k: usize| -> String {
            self.app
                .selected_session()
                .ao(k)
                .get(self.app.selected_solve_idx)
                .and_then(|r| *r)
                .map_or("-".to_string(), |d| format!("{:.3}", d.as_secs_f64()))
        };

        let (time_str, time_color) = match solve.penalty {
            Penalty::None => (
                format!("{:.3}", solve.effective_time().unwrap().as_secs_f64()),
                Color::Green,
            ),
            Penalty::PlusTwo => (
                format!("{:.3}+", solve.effective_time().unwrap().as_secs_f64()),
                Color::Yellow,
            ),
            Penalty::Dnf => ("DNF".to_string(), Color::Red),
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Min(0),
            ])
            .split(inner);

        BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .lines(vec![time_str.into()])
            .style(Style::default().fg(time_color).bold())
            .centered()
            .build()
            .render(layout[1], buf);

        let text = vec![
            line("AO5", ao_str(5), Color::Magenta),
            line("AO12", ao_str(12), Color::Magenta),
            line("Scramble", solve.scramble.to_string(), Color::White),
            line("Date", format_date(solve.timestamp), Color::Gray),
        ];

        Paragraph::new(text).render(layout[2], buf);
    }
}
