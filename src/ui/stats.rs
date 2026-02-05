use crate::{app::App, time_display::TimeDisplay};
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

fn span(duration: Option<Duration>, color: Color) -> Span<'static> {
    match duration {
        Some(d) => Span::styled(format!("{:<11}", d.format(3)), color),
        None => Span::styled(format!("{:<11}", "-"), color),
    }
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Stats")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let best = app.best_time();
    let worst = app.worst_time();
    let best_ao5 = app.ao(5).iter().filter_map(|&x| x).min();
    let best_ao12 = app.ao(12).iter().filter_map(|&x| x).min();
    let (count, sum): (usize, f64) = app
        .selected_session()
        .iter()
        .filter_map(|solve| solve.effective_time())
        .map(|d| d.as_secs_f64())
        .fold((0, 0.0), |(c, s), t| (c + 1, s + t));
    let average = (count > 0).then(|| Duration::from_secs_f64(sum / count as f64));
    let total = (count > 0).then(|| Duration::from_secs_f64(sum));

    let line1 = Line::from(vec![
        Span::raw(" "),
        Span::raw(format!("{:<12}", "Best:")),
        span(best, Color::Green),
        Span::raw(format!("{:<12}", "Worst:")),
        span(worst, Color::Red),
    ]);

    let line2 = Line::from(vec![
        Span::raw(" "),
        Span::raw(format!("{:<12}", "Best ao5:")),
        span(best_ao5, Color::Blue),
        Span::raw(format!("{:<12}", "Best ao12:")),
        span(best_ao12, Color::Cyan),
    ]);

    let line3 = Line::from(vec![
        Span::raw(" "),
        Span::raw(format!("{:<12}", "Average:")),
        span(average, Color::default()),
        Span::raw(format!("{:<12}", "Total:")),
        span(total, Color::default()),
    ]);

    let widget = Paragraph::new(vec![line1, line2, line3]).block(block);

    frame.render_widget(widget, area);
}
