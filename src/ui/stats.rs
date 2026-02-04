use crate::app::App;
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Stats")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let format_duration = |duration: Option<Duration>| match duration {
        Some(d) => format!("{:<11.2}", d.as_secs_f64()),
        None => format!("{:<11}", "-"),
    };

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

    let best_str = format_duration(best);
    let worst_str = format_duration(worst);
    let best_ao5_str = format_duration(best_ao5);
    let best_ao12_str = format_duration(best_ao12);
    let average_str = format_duration(average);
    let total_str = format_duration(total);

    let line1 = Line::from(vec![
        Span::raw(" "),
        Span::raw(format!("{:<12}", "Best:")),
        Span::styled(best_str, Style::default().fg(Color::Green)),
        Span::raw(format!("{:<12}", "Worst:")),
        Span::styled(worst_str, Style::default().fg(Color::Red)),
    ]);

    let line2 = Line::from(vec![
        Span::raw(" "),
        Span::raw(format!("{:<12}", "Best ao5:")),
        Span::styled(best_ao5_str, Style::default().fg(Color::Blue)),
        Span::raw(format!("{:<12}", "Best ao12:")),
        Span::styled(best_ao12_str, Style::default().fg(Color::Cyan)),
    ]);

    let line3 = Line::from(vec![
        Span::raw(" "),
        Span::raw(format!("{:<12}", "Average:")),
        Span::styled(average_str, Style::default()),
        Span::raw(format!("{:<12}", "Total:")),
        Span::styled(total_str, Style::default()),
    ]);

    let widget = Paragraph::new(vec![line1, line2, line3]).block(block);

    frame.render_widget(widget, area);
}
