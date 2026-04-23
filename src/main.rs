use anyhow::Result;
use crossterm::{
    event::{self, KeyboardEnhancementFlags},
    execute, terminal,
};

mod app;
mod config;
mod events;
mod scramble;
mod sessions;
mod time_display;
mod ui;

fn setup_keyboard_protocol() -> Result<()> {
    terminal::enable_raw_mode()?;
    if !terminal::supports_keyboard_enhancement()? {
        ratatui::restore();
        return Err(anyhow::anyhow!(
            "Terminal does not support keyboard enhancements"
        ));
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
        ui::draw(&app, &mut terminal)?;
        events::handle(&mut app)?;
    }
    Ok(())
}

fn handle_args() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "-v" || a == "--version") {
        println!("lazytimer {}", env!("CARGO_PKG_VERSION"));
        return true;
    }
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!(
            "Usage: lazytimer [OPTIONS]\n\
            \n\
            Options:\n\
              -h, --help     Print help information\n\
              -v, --version  Print version information"
        );
        return true;
    }
    false
}

fn main() {
    if handle_args() {
        return;
    }

    let result = run();
    ratatui::restore();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
