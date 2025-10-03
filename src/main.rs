use std::{io, thread, time::Duration};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders}, // components of rust TUIs are all widgets.
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    println!("This 'GetOff!' project is sure gonna be cool when it's done!");

    Ok(())
}
