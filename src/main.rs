use crossterm::{
    event::{self, KeyboardEnhancementFlags},
    execute, terminal,
};
use anyhow::Result;

mod app;
mod events;
mod scramble;
mod sessions;
mod ui;

fn setup_keyboard_protocol() -> Result<()> {
    terminal::enable_raw_mode()?;
    if !terminal::supports_keyboard_enhancement()? {
        ratatui::restore();
        return Err(anyhow::anyhow!("Terminal does not support keyboard enhancements"));
    }
    execute!(
        std::io::stdout(),
        event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    )?;
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new()?;
    setup_keyboard_protocol()?;

    while !app.exiting {
        ui::draw(&mut app, &mut terminal)?;
        events::handle(&mut app)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        ratatui::restore();
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    ratatui::restore();
}
