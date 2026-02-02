use crate::app::App;
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

pub struct Stats<'a> {
    app: &'a App,
}

impl<'a> Stats<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Stats<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Stats")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let format_duration = |duration: Option<Duration>| match duration {
            Some(d) => format!("{:<11.2}", d.as_secs_f64()),
            None => format!("{:<11}", "-"),
        };

        let best = self.app.best_time();
        let worst = self.app.worst_time();
        let best_ao5 = self.app.ao(5).iter().filter_map(|&x| x).min();
        let best_ao12 = self.app.ao(12).iter().filter_map(|&x| x).min();
        let solves = self.app.selected_session();
        let times: Vec<Duration> = solves
            .iter()
            .filter_map(|solve| solve.effective_time())
            .collect();
        let average = if !times.is_empty() {
            let sum: f64 = times.iter().map(|d| d.as_secs_f64()).sum();
            Some(Duration::from_secs_f64(sum / times.len() as f64))
        } else {
            None
        };
        let total = if !times.is_empty() {
            Some(times.iter().sum())
        } else {
            None
        };

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

        let text = vec![line1, line2, line3];

        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
