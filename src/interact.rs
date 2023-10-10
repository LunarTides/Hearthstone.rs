/*use std::{
    io::{self, Stderr},
    time::Duration,
};

use anyhow::Result;
use crossterm::event;
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph, Terminal};

type TerminalType = Terminal<CrosstermBackend<Stderr>>;

fn startup() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    Ok(())
}

fn shutdown() -> Result<()> {
    crossterm::execute!(io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn create_terminal() -> Result<TerminalType> {
    let terminal = Terminal::new(CrosstermBackend::new(io::stderr()))?;
    Ok(terminal)
}

pub fn main() -> Result<()> {
    startup()?;
    let mut terminal = create_terminal()?;

    loop {
        terminal.draw(|f| {
            f.render_widget(Paragraph::new("Hello World! (press 'q' to quit)"), f.size());
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Char('Q') {
                    break;
                }
            }
        }
    }

    shutdown()?;

    Ok(())
}
*/
