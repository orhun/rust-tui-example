mod event;

use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use event::{Event, EventHandler};
use std::io;
use tui::backend::CrosstermBackend;
use tui::widgets::Paragraph;
use tui::Terminal;

fn main() -> Result<()> {
    /* 1- Initialize the terminal */
    let backend = CrosstermBackend::new(io::stderr());
    let mut terminal = Terminal::new(backend)?;

    /* 2- Create an event handler thread */
    let event_handler = EventHandler::new(250);

    /* 3- Prepare the terminal for rendering */
    terminal::enable_raw_mode()?;
    crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    /* 4- Start the render loop */
    let mut running = true;
    while running {
        /* 4.1- Render widgets */
        terminal.draw(|frame| frame.render_widget(Paragraph::new("rust munich"), frame.size()))?;

        /* 4.2- Handle events */
        match event_handler.next()? {
            Event::Key(key_event) => match key_event.code {
                // exit on ESC key press
                KeyCode::Esc => {
                    running = false;
                }
                _ => {}
            },
            _ => {}
        }
    }

    /* 5- Flush the terminal before exit */
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
