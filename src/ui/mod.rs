use anyhow::Result;
use crate::app::{App, Screen};
use ratatui::DefaultTerminal;

mod statistics_screen;
mod timer_screen;

pub fn draw(app: &mut App, terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.draw(|frame| {
        match app.current_screen {
            Screen::Timer => {
                frame.render_widget(timer_screen::TimerScreen::new(app), frame.area())
            }
            Screen::Statistics => frame.render_widget(
                statistics_screen::StatisticsScreen::new(app),
                frame.area(),
            ),
        };
    })?;
    Ok(())
}
