use crate::App;
use crate::TriggerAction;
use ratatui::widgets::BorderType;
use simple_string_patterns::StripCharacters;

use ratatui::{
    Frame,
    layout::*,
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
        .padding(Padding::new(0, 0, frame.area().height / 4, 0));

    // format TimeDelta info for calculations
    let total_secs_left = app
        .time_left()
        .to_string()
        .strip_non_digits()
        .parse::<i64>()
        .unwrap();

    // calculate hrs/mins/secs left
    let hrs_left = total_secs_left / 3600;
    let mins_left = (total_secs_left % 3600) / 60;
    let secs_left = total_secs_left % 60;

    // get appropriate trigger text
    let trigger = match app.trigger_action {
        TriggerAction::Shutdown => "Shutdown",
        _ => "",
    };

    // format pieces to pass to the BigText builder
    let hrs_str = format!("{hrs_left} hours,");
    let mins_str = format!("{mins_left} minutes,");
    let secs_str = format!("{secs_left} seconds");
    let trigger_str = format!("{trigger}");

    let timer_paragraph = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
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

    // get smaller area for big text rendering based on parent block padding
    let big_text_area = timer_block.inner(frame.area());

    frame.render_widget(timer_block, frame.area()); // render outline
    frame.render_widget(timer_paragraph, big_text_area); // render timer
}

// TODO:
// Fix prompt centering
// Current implementation is poor. Find way to implement
// examples provided in ratatui docs:
// https://ratatui.rs/recipes/layout/center-a-widget/
fn render_popup(frame: &mut Frame, app: &mut App) {
    // should have bool which indicates whether or not this should be rendered.
    if app.timer_input_prompt {
        let border_block = Block::bordered()
            .style(Style::default())
            .padding(Padding::new(0, 0, frame.area().height / 2, 0))
            .title_top("Time Entry");

        frame.render_widget(&border_block, frame.area());

        // Make centered rect that prompt will live in
        // static pixel subtraction is bad but will do for now.
        let prompt_rect = Rect::new(frame.area().width / 3, frame.area().height / 2, 50, 5);

        // Same rect with slightly different offset to surround prompt
        let prompt_border_rect =
            Rect::new(frame.area().width / 3, frame.area().height / 2 - 2, 50, 5);

        // block design for prompt
        let prompt_block = Block::bordered()
            .style(Style::default())
            .border_type(BorderType::Rounded);

        // Render the border for the prompt
        frame.render_widget(prompt_block, prompt_border_rect);

        // Render the prompt itself.
        TextPrompt::from("Please enter desired time in minutes:").draw(
            frame,
            prompt_rect,
            &mut app.timer_length_state,
        );
    }
}
