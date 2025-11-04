use crate::App;
use crate::TriggerAction;
use ratatui::widgets::BorderType;
use ratatui::widgets::Paragraph;
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
    // let timer_block = Block::bordered()
    //     .style(Style::default())
    //     .padding(Padding::new(0, 0, frame.area().height / 4, 0));
    let timer_block = Block::bordered().style(Style::default());

    let total_secs_left = app.time_left();

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

    // note that height/width is in COLUMNS AND ROWS, not pixels!
    // TODO:: Add a check for smaller than like 5-6 height that displays
    // a message saying the terminal is too small.
    // TODO: Refactor this, it's waaaaaaay more complicated than it needs to be
    if frame.area().height < 14 {
        let timer_bigtext = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .centered()
            .style(Style::default())
            //.lines(vec![hrs_str.into(), mins_str.into(), secs_str.into()])
            .lines(vec![
                format!("{hrs_left}:{mins_left}:{secs_left}").into(),
                String::from("remaining").into(),
            ])
            .build();

        // get smaller area for big text rendering based on parent block padding,
        // and make sure it's centered in the window
        let big_text_area = center(
            timer_block.inner(frame.area()),
            Constraint::Length(60),
            Constraint::Length(15),
        );

        frame.render_widget(timer_block, frame.area()); // render outline
        frame.render_widget(timer_bigtext, big_text_area); // render timer
    } else if frame.area().height < 25 {
        let timer_bigtext = BigText::builder()
            .pixel_size(PixelSize::Quadrant)
            .centered()
            .style(Style::default())
            .lines(vec![hrs_str.into(), mins_str.into(), secs_str.into()])
            .build();

        // get smaller area for big text rendering based on parent block padding,
        // and make sure it's centered in the window
        let big_text_area = center(
            timer_block.inner(frame.area()),
            Constraint::Length(60),
            Constraint::Length(15),
        );

        frame.render_widget(timer_block, frame.area()); // render outline
        frame.render_widget(timer_bigtext, big_text_area); // render timer
    } else {
        let timer_bigtext = BigText::builder()
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

        // get smaller area for big text rendering based on parent block padding,
        // and make sure it's centered in the window
        //let big_text_area = timer_block.inner(frame.area());

        let big_text_area = center(
            timer_block.inner(frame.area()),
            Constraint::Length(60),
            Constraint::Length(30),
        );

        frame.render_widget(timer_block, frame.area()); // render outline
        frame.render_widget(timer_bigtext, big_text_area); // render timer
    }
}

fn render_popup(frame: &mut Frame, app: &mut App) {
    // timer input prompt bool used to check whether popup should be
    // shown.
    if app.timer_input_prompt {
        let border_block = Block::bordered()
            .style(Style::default())
            //.padding(Padding::new(0, 0, frame.area().height / 2, 0))
            .title_top("Time Entry");

        frame.render_widget(&border_block, frame.area());

        // block design for prompt
        let prompt_block = Block::bordered()
            .style(Style::default())
            .border_type(BorderType::Rounded)
            .title_top("Please enter desired time in minutes:");

        // 2 rects: border is differently sized in order to surround prompt
        let prompt_rect = center(frame.area(), Constraint::Length(50), Constraint::Length(1));
        let prompt_border_rect =
            center(frame.area(), Constraint::Length(60), Constraint::Length(5));

        // render the border around the prompt
        frame.render_widget(prompt_block, prompt_border_rect);

        // Render the prompt itself.
        TextPrompt::from("").draw(frame, prompt_rect, &mut app.timer_length_state);
    }
}

// ratatui docs provided widget centering solution
// area = area whose center will be found
fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
