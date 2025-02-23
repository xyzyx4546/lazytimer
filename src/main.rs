use crossterm::{
    event::{self, KeyboardEnhancementFlags},
    execute, terminal,
};
use std::io::{stdout, Result};
use std::process::exit;

mod app;
mod events;
mod ui;

fn setup_keyboard_protocol() -> Result<()> {
    terminal::enable_raw_mode()?;

    if !terminal::supports_keyboard_enhancement()? {
        ratatui::restore();
        eprintln!("Error: Terminal does not support keyboard enhancements");
        exit(1);
    }

    execute!(
        stdout(),
        event::PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    setup_keyboard_protocol()?;


    while !app.exiting {
        ui::draw(&app, &mut terminal)?;
        events::handle(&mut app)?;
    }

    ratatui::restore();
    Ok(())
}
