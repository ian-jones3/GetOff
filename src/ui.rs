use crate::App;
use ratatui::{
    Frame,
    layout::*,
    style::{self, Style},
    symbols::line::VERTICAL,
    widgets::{Block, Borders, Paragraph},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let block_text = Paragraph::new("GetOff!").block(title_block);

    let title_layout = Layout::default()
        .direction(Direction::Vertical)
        //.vertical_margin(10)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Render title
    frame.render_widget(block_text, title_layout[0]);

    let user_prompt_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let user_prompt_text =
        Paragraph::new("Please enter amount of time in minutes").block(user_prompt_block);

    frame.render_widget(user_prompt_text, title_layout[1]);

    let controls_instruction_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let controls_instruction_text = Paragraph::new("-- q: Exit program --")
        .centered()
        .block(controls_instruction_block);

    frame.render_widget(controls_instruction_text, title_layout[2]);
}
