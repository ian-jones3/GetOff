mod app;
mod ui;
use app::*;
use ui::*;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{Terminal, prelude::Backend};
use std::error::Error;
use std::io::{self};

fn main() -> Result<(), Box<dyn Error>> {
    // initialize a new DefaultTerminal
    let mut terminal = ratatui::init();

    // Start core application cycle
    let mut app = App::new();
    let mut app_result = run_app(&mut terminal, &mut app);

    // restore terminal to original state and return
    ratatui::restore();
    Ok(())
}

// Main loop
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        //terminal.draw(|f| ui(f, app))
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        println!("Quit out detected, performing a clean exit:");
                        break;
                    }
                    KeyCode::Char(_) => {
                        ratatui::restore();
                        App::execute_shutdown();
                    }
                    _ => println!("not a keypress, ignoring"),
                }
            }
        }
    }
    Ok(())
}
