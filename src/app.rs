use chrono::TimeDelta;
use std::sync::Arc;
use std::{
    io::{self, Error},
    process::exit,
};
use system_shutdown::shutdown;
use timer::Timer;

pub enum AppState {
    Title,
    TimerInput,
    Exit,
}

pub enum EditableValue {
    Timer,
}

pub struct App {
    pub current_state: AppState,
    pub timer: Timer,
    pub timer_length: Option<TimeDelta>,
    pub editing: Option<EditableValue>,
    pub shutdown: bool,
}

impl App {
    /// new()
    /// Instantiate the App state.
    pub fn new() -> App {
        App {
            current_state: AppState::Title,
            timer: Timer::new(),
            timer_length: None,
            editing: None,
            shutdown: false,
        }
    }

    pub fn set_timer(mut self, time_in_mins: i64) -> Result<(), io::Error> {
        // in long term, should check to make sure requested timer is not
        // absurdly long. If a very long timer is detected, logically would
        // flip a flag that causes a pop up to be drawn asking if user really
        // wants such a long timer.

        // ensure that time in mins i64 is never negative.
        // The reason we're not using u64 is because TimeDelta minutes() takes
        // an i64.

        self.timer_length = Some(TimeDelta::minutes(time_in_mins));
        Ok(())
    }

    // seperate from set because user may change mind on what they
    // want timer to be, so we will wait until they confirm to start.
    //
    // Might need a return type for error handling, need to look into
    // how timer operations work.
    pub fn start_timer(self: &Arc<Self>) {
        match &self.timer_length {
            Some(time_delta) => {
                self.timer
                    .schedule_with_delay(*time_delta, || App::execute_shutdown());
            }
            None => {
                eprint!("ERROR: ATTEMPTED TO START TIMER WITH NO DURATION SET");
            }
        }
    }

    pub fn execute_shutdown() {
        println!("Shutdown sequence successfully executed");
        exit(0)

        // Below code will actually shut down the computer, do not use in testing
        // unless running through a VM!
        // match shutdown() {
        //     Ok(_) => println!("Successfully shutting down"),
        //     Err(error) => println!("Shutdown failure, Error: {error}"),
        // }
    }
}
