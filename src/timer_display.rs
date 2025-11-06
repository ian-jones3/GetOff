use crate::App;
use crate::TriggerAction;
use ratatui::widgets::BorderType;

use ratatui::{Frame, layout::*, style::Style, widgets::Block};
use tui_widgets::big_text::*;
use tui_widgets::prompts::*;

// -----------------------------
// render_timer_display
// Public function called by the app in order to render the
// timer display. Will do nothing if no timer is set.
pub fn render_timer_display(frame: &mut Frame, app: &mut App) {
    render_popup(frame, app);

    // Do not attempt to render the timer unless the timer_length
    // variable has been set.
    match app.timer_length {
        Some(_) => render_timer(frame, app),
        None => {}
    }
}

// -----------------------------
// render_timer
// Will render the text of the timer using BigText.
fn render_timer(frame: &mut Frame, app: &App) {
    // Create the block the timer text will be inside
    let timer_block = Block::bordered().style(Style::default());

    // Fetch time left from app state
    let total_secs_left = app.time_left();

    // calculate hrs/mins/secs left
    let hrs_left = total_secs_left / 3600;
    let mins_left = (total_secs_left % 3600) / 60;
    let secs_left = total_secs_left % 60;

    // get appropriate trigger text
    let trigger = match app.trigger_action {
        TriggerAction::Shutdown => "Shutdown",
        // More cases will be added in future as more
        // Trigger actions are implemented.
        _ => "",
    };

    // format pieces to pass to the BigText builder
    let hrs_str = format!("{hrs_left} hours,");
    let mins_str = format!("{mins_left} minutes,");
    let secs_str = format!("{secs_left} seconds");
    let trigger_str = format!("{trigger}");

    // TODO:: Add a check for smaller than like 5-6 height that displays
    // a message saying the terminal is too small.

    // Create the big text builder we will use to create our BigText
    let mut big_text_binding = BigText::builder();
    let timer_bigtext: &mut BigTextBuilder<'_> = big_text_binding
        .pixel_size(PixelSize::Quadrant)
        .centered()
        .style(Style::default());

    let text: BigText<'_>; // the timer text

    // area the display text will be in
    let mut big_text_area: Rect = center(
        timer_block.inner(frame.area()),
        Constraint::Length(60),
        Constraint::Length(15),
    );

    // note that height/width is in COLUMNS AND ROWS, not pixels!
    if frame.area().height < 14 {
        text = timer_bigtext
            .lines(vec![
                format!("{hrs_left}:{mins_left}:{secs_left}").into(),
                String::from("remaining").into(),
            ])
            .build();
    } else if frame.area().height < 25 {
        text = timer_bigtext
            .lines(vec![hrs_str.into(), mins_str.into(), secs_str.into()])
            .build();
    } else {
        text = timer_bigtext
            .lines(vec![
                hrs_str.into(),
                mins_str.into(),
                secs_str.into(),
                String::from("remaining").into(),
                String::from("until").into(),
                trigger_str.into(),
            ])
            .build();

        // Different constraint needed for this type of display
        // To center cleanly.
        big_text_area = center(
            timer_block.inner(frame.area()),
            Constraint::Length(60),
            Constraint::Length(30),
        );
    }

    // Execute the renders.
    frame.render_widget(timer_block, frame.area()); // render outline
    frame.render_widget(text, big_text_area); // render timer
}

// -----------------------------
// render_popup
// Renders the prompt in which the user can enter their desired timer.
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

        // 2 rects: prompt_border_rect is differently sized in order to surround prompt_rect
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
// area = area whose center will be found i.e frame.area()
fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
