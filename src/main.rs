mod app;
mod ui;
use app::*;
use tui_input::backend::crossterm::EventHandler;
use ui::*;

// Remember to use crossterm through ratatui's crate!
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
};

use std::error::Error;

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
        // Draw the frame
        let draw_result = terminal.draw(|f| render_ui(f, app));
        match draw_result {
            Ok(_) => {}
            Err(error) => panic!("ERROR: FAILED TO DRAW FRAME: {error}"),
        }

        // Key event handling
        let event = event::read()?;
        if let Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Press {
                match app.input_mode {
                    InputMode::NotEditing => match key.code {
                        KeyCode::Char('q') => {
                            println!("Quit out detected, performing a clean exit:");
                            break;
                        }
                        KeyCode::Char('i') => {
                            app.edit();
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => app.stop_edit(),
                        KeyCode::Enter => {
                            let time = app.input.value().parse::<i64>()?;
                            app.set_timer(time)?;
                            app.start_timer();
                            app.stop_edit();
                        }
                        // by nesting this handle_event call in braces
                        // and using ; we can contain the return inside
                        // this scope, preventing a type mismatch in the
                        // match statement.
                        _ => {
                            app.input.handle_event(&event);
                        }
                    },
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Check that terminal.draw doesn't error out
    // when using our ui function
    fn draw_works() {}
}
