use crate::{App, app::AppState, app_site_screen, timer_display, title_screen};
use ratatui::Frame;

pub fn render_ui(frame: &mut Frame, app: &mut App) {
    match app.current_state {
        AppState::Title => title_screen::render_title_screen(frame, app),
        AppState::TimerDisplay => timer_display::render_timer_display(frame, app),
        AppState::AppSiteSelection => app_site_screen::render_application_list(frame, app),
        _ => (),
    }
}
