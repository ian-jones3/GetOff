use std::rc::Rc;

use crate::App;
use ratatui::{
    Frame,
    layout::*,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};
use tui_widgets::prompts::*;

pub fn render_timer_display(frame: &mut Frame, app: &mut App) {
    let layout = build_layout(frame);
    render_popup(frame, app, &layout);

    // put a match statement to make sure a timer is running before
    // this is called, currently it is triggering panic in App.rs
    match app.timer_length {
        Some(_) => render_timer(frame, app, &layout),
        None => {}
    }

    //todo!()
}

fn build_layout(frame: &Frame) -> Rc<[Rect]> {
    // ultra simple layout
    let layout = Layout::vertical([Constraint::Percentage(100)]).split(frame.area());
    layout
}

fn render_timer(frame: &mut Frame, app: &App, layout: &Rc<[Rect]>) {
    let timer_block = Block::bordered()
        .title_top("Timer Remaining")
        .style(Style::default());
    let timer_paragraph = Paragraph::new(app.time_left().to_string()).block(timer_block);
    frame.render_widget(timer_paragraph, frame.area());
}

fn render_popup(frame: &mut Frame, app: &mut App, layout: &Rc<[Rect]>) {
    // should have bool which indicates whether or not this should be rendered.
    if app.timer_input_prompt {
        TextPrompt::from("Enter desired time").draw(
            frame,
            frame.area(),
            &mut app.timer_length_state,
        );
    }
}
