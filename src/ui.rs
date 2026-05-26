use crate::app::AppState;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::io;

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }

    pub fn draw<F: FnOnce(&mut Frame)>(&mut self, f: F) -> io::Result<()> {
        let mut term = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;
        term.draw(f)?;
        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm::execute!(io::stdout(), LeaveAlternateScreen);
        let _ = crossterm::terminal::disable_raw_mode();
    }
}

pub fn draw(frame: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let left = Paragraph::new(vec![
        Line::from(vec![Span::styled("Portfolio", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from(format!("PnL: {:+.1}%", state.pnl_pct)),
        Line::from(format!("Exposure: {:.0}%", state.exposure_pct)),
        Line::from(format!("VaR(95%): ${:.0}", state.var_95)),
    ])
    .block(Block::default().title("Risk").borders(Borders::ALL));

    let right = Paragraph::new(vec![
        Line::from("Order Entry"),
        Line::from(format!("Symbol: {}", state.symbol)),
        Line::from("[o] new order  [x] cancel  [k] kill switch"),
    ])
    .block(Block::default().title("Trading").borders(Borders::ALL));

    frame.render_widget(left, chunks[0]);
    frame.render_widget(right, chunks[1]);
}
