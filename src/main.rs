mod app;
mod app_selection;
mod app_site_screen;
mod timer_display;
mod title_screen;
mod ui;
use app::*;
use tui_widgets::prompts::State;
use ui::*;

use clap::{Arg, Command, Parser};

// Remember to use crossterm through ratatui's crate!
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
};

use std::error::Error;
use std::time::Duration;

use crate::app_selection::build_app_list;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Go direct to timer, skipping title
    #[clap(short = 't', long = "timer")]
    pub t: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    //only here for quick and dirty testing
    //
    //build_app_list();

    // initialize a new DefaultTerminal
    let mut terminal = ratatui::init();

    // Start core application cycle
    let mut app = App::new();

    handle_flags(args, &mut app);

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

        // TODO:
        // Rework key event handling to be cleaner, move out of
        // this file if possible
        // Edit/NotEditing dynamic is no longer satisfactory,
        // need to add more states like for using arrow keys
        // on list views
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
                            KeyCode::Char('a') => {
                                app.current_state = AppState::AppSiteSelection;
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

fn handle_flags(args: Args, app: &mut App) {
    if args.t {
        app.timer_input_prompt = true;
        app.current_state = AppState::TimerDisplay;
        app.edit();
    }
}
