use std::rc::Rc;

use crate::{App, app::AppState, timer_display, title_screen};
use ratatui::{
    Frame,
    layout::*,
    style::{self, Style},
    widgets::{Block, Borders, Paragraph, block::Title},
};

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    match app.current_state {
        AppState::Title => title_screen::render_title_screen(frame, app),
        AppState::TimerDisplay => {
            timer_display::render_timer_display(frame, app);
        }
        _ => (),
    }
}
