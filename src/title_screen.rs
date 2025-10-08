use std::rc::Rc;

use crate::App;
use ratatui::{
    Frame,
    layout::*,
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph},
};

pub fn render_title_screen(frame: &mut Frame, app: &App) {
    let layout = build_layout(frame);
    render_ascii(frame, app, &layout);
    render_button_instructions(frame, app, &layout);
}

fn build_layout(frame: &Frame) -> Rc<[Rect]> {
    // centered title in ascii art, list of instructions on how
    // each
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // Title
            Constraint::Length(3), // Enter timer
            Constraint::Length(3), // Enter applications/websites
            Constraint::Length(3), // App/site lists
            Constraint::Length(3), // Timer lists
            Constraint::Length(3), // Exit
        ])
        .split(frame.area());
    layout
}

fn render_ascii(frame: &mut Frame, app: &App, layout: &Rc<[Rect]>) {
    let ascii_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .padding(Padding::new(0, 0, frame.area().height / 5, 0));
    // generated using https://www.asciiart.eu/text-to-ascii-art
    let ascii_text = Paragraph::new(
        r"


   █████████            █████       ███████       ██████     ██████  ███
  ███░░░░░███          ░░███      ███░░░░░███    ███░░███   ███░░███░███
 ███     ░░░   ██████  ███████   ███     ░░███  ░███ ░░░   ░███ ░░░ ░███
░███          ███░░███░░░███░   ░███      ░███ ███████    ███████   ░███
░███    █████░███████   ░███    ░███      ░███░░░███░    ░░░███░    ░███
░░███  ░░███ ░███░░░    ░███ ███░░███     ███   ░███       ░███     ░░░ 
 ░░█████████ ░░██████   ░░█████  ░░░███████░    █████      █████     ███
  ░░░░░░░░░   ░░░░░░     ░░░░░     ░░░░░░░     ░░░░░      ░░░░░     ░░░ 

",
    )
    .centered()
    .block(ascii_block);

    frame.render_widget(ascii_text, layout[0]);
}

// Button instructions will all be the same, so they'll share a
// block design (if possible). could potentially make this a loop
// rendering the lines matching them up to their spot in the layout.
fn render_button_instructions(frame: &mut Frame, app: &App, layout: &Rc<[Rect]>) {
    let button_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    // build list of buttons that will be added
    // To add buttons, simply add to this list of strings.
    let buttons = vec![
        "T: Set a timer",
        "A: Choose applications/websites",
        "L: Configure App/Site lists",
        "O: Configure Timer lists",
        "Q: quit",
    ];

    // track what spot in the layout it will take up
    let mut layout_spot = 1;
    // iterate building the buttons
    for button in buttons {
        frame.render_widget(
            build_button_paragraph(String::from(button), &button_block),
            layout[layout_spot],
        );
        layout_spot = layout_spot + 1;
    }
}

// Will create a button for the home screen using the string and block design
// passed to it.
fn build_button_paragraph<'a>(instruction: String, design: &Block<'a>) -> Paragraph<'a> {
    // cloning not ideal but performance SHOULD be acceptable
    let button_para = Paragraph::new(instruction).centered().block(design.clone());
    button_para
}
