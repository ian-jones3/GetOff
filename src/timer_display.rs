use crate::App;
use crate::TriggerAction;
use simple_string_patterns::StripCharacters;

use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Padding},
};
use tui_widgets::big_text::*;
use tui_widgets::prompts::*;

pub fn render_timer_display(frame: &mut Frame, app: &mut App) {
    render_popup(frame, app);

    // put a match statement to make sure a timer is running before
    // this is called, currently it is triggering panic in App.rs
    match app.timer_length {
        Some(_) => render_timer(frame, app),
        None => {}
    }
}

// Will need refactoring when we start accounting for
// a variety of window sizes
fn render_timer(frame: &mut Frame, app: &App) {
    let timer_block = Block::bordered()
        .style(Style::default())
        .padding(Padding::new(0, 0, 2, 0));

    let total_secs_left = app
        .time_left()
        .to_string()
        .strip_non_digits()
        .parse::<i64>()
        .unwrap();
    let hrs_left = total_secs_left / 3600;
    let mins_left = total_secs_left / 60;
    let secs_left = total_secs_left % 60;

    let trigger = match app.trigger_action {
        TriggerAction::Shutdown => "Shutdown",
        _ => "",
    };

    let hrs_str = format!("{hrs_left} hours,");
    let mins_str = format!("{mins_left} minutes,");
    let secs_str = format!("{secs_left} seconds");
    let trigger_str = format!("{trigger}");

    let timer_paragraph = BigText::builder()
        .pixel_size(PixelSize::Full)
        .centered()
        .style(Style::default())
        .lines(vec![
            hrs_str.into(),
            mins_str.into(),
            secs_str.into(),
            String::from("remaining").into(),
            String::from("until").into(),
            trigger_str.into(),
        ])
        .build();

    // let timer_paragraph = Paragraph::new(timer_paragraph_text)
    //     .centered()
    //     .block(timer_block);

    // get smaller area for big text rendering based on parent block padding
    let big_text_area = timer_block.inner(frame.area());

    frame.render_widget(timer_block, frame.area());
    frame.render_widget(timer_paragraph, big_text_area);
}

fn render_popup(frame: &mut Frame, app: &mut App) {
    // should have bool which indicates whether or not this should be rendered.
    if app.timer_input_prompt {
        TextPrompt::from("Enter desired time").draw(
            frame,
            frame.area(),
            &mut app.timer_length_state,
        );
    }
}
