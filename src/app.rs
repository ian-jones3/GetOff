use chrono::TimeDelta;
use std::{
    io::{self, Error},
    process::exit,
    time::Duration,
};
use tui_widgets::prompts::TextState;

use std::time::Instant;
use system_shutdown::shutdown;
use timer::Timer;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

pub enum AppState {
    Title,
    TimerDisplay,
    Exit,
}

pub enum EditableValue {
    Timer,
    Applications,
    Websites,
    SavedApps,
    SavedWebsites,
}

pub enum InputMode {
    Editing,
    NotEditing,
}

pub enum TriggerAction {
    Shutdown,
    Restart,
    Close,
    Warn,
}

pub struct App<'a> {
    pub current_state: AppState,
    pub timer: Timer,                    // Timer object
    pub timer_length: Option<TimeDelta>, // timer length set by user
    pub editing: Option<EditableValue>,
    pub input: Input,          // not in use
    pub input_mode: InputMode, // tracks if user is typing
    pub timer_length_state: TextState<'a>,
    // will need to add more state like this for
    // applications and websites
    pub timer_input_prompt: bool,
    pub start_time: Option<Instant>, // track when timer started
    pub trigger_action: TriggerAction,
}

impl<'a> App<'a> {
    /// new()
    /// Instantiate the App state.
    pub fn new() -> App<'a> {
        App {
            current_state: AppState::Title,
            timer: Timer::new(),
            timer_length: None,
            editing: None,
            input: Input::default(),
            input_mode: InputMode::NotEditing,
            timer_length_state: TextState::default(),
            timer_input_prompt: false,
            start_time: None,
            trigger_action: TriggerAction::Shutdown,
        }
    }

    pub fn set_timer(&mut self, time_in_mins: i64) -> Result<(), io::Error> {
        // in long term, should check to make sure requested timer is not
        // absurdly long. If a very long timer is detected, logically would
        // flip a flag that causes a pop up to be drawn asking if user really
        // wants such a long timer.
        // For now, just a very simple check that prints to terminal.
        match time_in_mins {
            ..=0 => return Err(Error::other("ERROR: NEGATIVE/0 VAL PASSED TO SET_TIMER")),
            600.. => println!("Wow! {time_in_mins} is a very long time!"),
            _ => self.timer_length = Some(TimeDelta::minutes(time_in_mins)),
        }

        Ok(())
    }

    // seperate from set because user may change mind on what they
    // want timer to be, so we will wait until they confirm to start.
    //
    // Might need a return type for error handling, need to look into
    // how timer operations work.
    pub fn start_timer(self: &mut Self) {
        match &self.timer_length {
            Some(time_delta) => {
                self.timer
                    .schedule_with_delay(*time_delta, || App::execute_shutdown())
                    .ignore(); // ignore the guard so the timer doesn't cancel
                //self.current_state = AppState::TimerDisplay;
                self.start_time = Some(Instant::now());
            }
            None => {
                eprint!("ERROR: ATTEMPTED TO START TIMER WITH NO DURATION SET");
            }
        }
    }

    pub fn time_left(&self) -> i64 {
        match &self.timer_length {
            Some(time_delta) => {
                let total_seconds = time_delta.num_seconds();
                let elapsed_seconds = self.start_time.unwrap().elapsed().as_secs() as i64;
                let remaining_seconds = total_seconds - elapsed_seconds;

                remaining_seconds
            }
            None => {
                eprint!("ERROR: ATTEMPTED TO RETURN TIME LEFT WHEN NO TIMER RUNNING");
                panic!()
            }
        }
    }

    pub fn execute_shutdown() {
        println!("Shutdown sequence successfully executed");
        // ratatui::restore();
        // exit(0);

        // Below code will actually shut down the computer, do not use in testing
        // unless running through a VM!
        match shutdown() {
            Ok(_) => println!("Successfully shutting down"),
            Err(error) => println!("Shutdown failure, Error: {error}"),
        }
    }

    pub fn edit(&mut self) {
        self.input_mode = InputMode::Editing;
    }

    pub fn stop_edit(&mut self) {
        self.input_mode = InputMode::NotEditing;
    }
}
