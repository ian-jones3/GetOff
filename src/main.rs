mod app;
mod timer_display;
mod title_screen;
mod ui;
use app::*;
use tui_widgets::prompts::State;
use ui::*;

// Remember to use crossterm through ratatui's crate!
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
};

use std::error::Error;
use std::time::Duration;

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
        if event::poll(Duration::from_millis(15))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match app.input_mode {
                        InputMode::NotEditing => match key.code {
                            KeyCode::Char('q') => {
                                println!("Quit out detected, performing a clean exit:");
                                break;
                            }
                            KeyCode::Char('t') => {
                                app.timer_input_prompt = true;
                                app.current_state = AppState::TimerDisplay;
                                app.edit();
                            }
                            _ => {}
                        },
                        InputMode::Editing => match key.code {
                            KeyCode::Esc => app.stop_edit(),
                            KeyCode::Enter => {
                                // pass state to set_timer
                                let time = app.timer_length_state.value().parse::<i64>()?;
                                app.set_timer(time)?;
                                app.start_timer();
                                app.stop_edit();
                                app.timer_input_prompt = false;
                            }
                            // by nesting this handle_event call in braces
                            // and using ; we can contain the return inside
                            // this scope, preventing a type mismatch in the
                            // match statement.
                            _ => {
                                // only functional for modifying timer state
                                // currently
                                app.timer_length_state.handle_key_event(key);
                            }
                        },
                    }
                }
            }
        }
    }
    Ok(())
}
