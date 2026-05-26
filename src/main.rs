mod app;
mod connectors;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use std::time::Duration;

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let mut terminal = ui::TerminalGuard::new()?;
    let mut state = app::AppState::default();

    loop {
        terminal.draw(|f| ui::draw(f, &state))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => state.refresh_portfolio(),
                    KeyCode::Char('k') => state.kill_switch = true,
                    _ => {}
                }
            }
        }
        if state.kill_switch {
            break;
        }
    }
    Ok(())
}
