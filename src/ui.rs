use std::rc::Rc;

use crate::App;
use chrono::TimeDelta;
use ratatui::{
    Frame,
    layout::*,
    style::{self, Style},
    symbols::line::VERTICAL,
    widgets::{Block, Borders, Paragraph, block::Title},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

pub fn render_ui(frame: &mut Frame, app: &App) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let block_text = Paragraph::new("GetOff!").centered().block(title_block);

    let title_layout = Layout::default()
        .direction(Direction::Vertical)
        //.vertical_margin(10)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Render title
    frame.render_widget(block_text, title_layout[0]);

    // Render request for user input
    let user_prompt_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let user_prompt_text =
        Paragraph::new("Please enter amount of time in minutes").block(user_prompt_block);

    frame.render_widget(user_prompt_text, title_layout[1]);

    // Render user input
    render_user_input(frame, app, &title_layout);

    render_time_left(frame, app, &title_layout);

    let controls_instruction_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let controls_instruction_text = Paragraph::new("-- q: Exit program --")
        .centered()
        .block(controls_instruction_block);

    frame.render_widget(controls_instruction_text, title_layout[4]);
}

fn render_time_left(frame: &mut Frame, app: &App, layout: &Rc<[Rect]>) {
    let time = match app.timer_length {
        Some(delta) => (delta.as_seconds_f32() / 60.0).to_string(),
        None => String::from("No timer started."),
    };
    let timer_display = Paragraph::new(time)
        .style(Style::default())
        .block(Block::bordered().title("Time left in minutes"));
    frame.render_widget(timer_display, layout[2]);
}

fn render_user_input(frame: &mut Frame, app: &App, layout: &Rc<[Rect]>) {
    let input_display = Paragraph::new(app.input.value())
        .style(Style::default())
        .block(Block::bordered().title("User Input"));
    frame.render_widget(input_display, layout[3]);
}
