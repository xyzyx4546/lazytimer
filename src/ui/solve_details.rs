use crate::{app::App, sessions::Penalty};
use ratatui::{prelude::*, widgets::*};
use std::time::{SystemTime, UNIX_EPOCH};
use tui_widgets::big_text::*;

fn line(key: &str, value: String, color: Color) -> Line<'_> {
    Line::from(vec![
        Span::styled(
            format!("{:>10}  ", key),
            Style::default().fg(Color::Magenta),
        ),
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

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(format!("Solve #{}", app.selected_solve_idx + 1))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let solve = app.selected_solve().unwrap();

    let ao_str = |k: usize| -> String {
        app.ao(k)
            .get(app.selected_solve_idx)
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

    let big_text = BigText::builder()
        .lines(vec![time_str.into()])
        .centered()
        .style(Style::default().fg(time_color).bold())
        .pixel_size(PixelSize::Sextant)
        .build();

    let text = vec![
        line("AO5", ao_str(5), Color::Blue),
        line("AO12", ao_str(12), Color::Cyan),
        line("Scramble", solve.scramble.to_string(), Color::White),
        line("Date", format_date(solve.timestamp), Color::DarkGray),
    ];

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(4),
        Constraint::Min(0),
    ])
    .split(block.inner(area));

    frame.render_widget(block, area);
    frame.render_widget(big_text, layout[1]);
    frame.render_widget(Paragraph::new(text), layout[2]);
}
