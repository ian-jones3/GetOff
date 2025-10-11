use crate::App;
use ratatui::{
    Frame,
    layout::*,
    style::Style,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, StatefulWidget},
};

pub fn render_application_list(frame: &mut Frame, app: &mut App) {
    // unwrap is ok here, if app list building fails something is
    // fundamentally wrong.
    let applications: Vec<ListItem> = app
        .application_list
        .applications
        .iter()
        .enumerate()
        .map(|(i, application)| ListItem::from(application.to_string()))
        .collect();

    let l_widget = List::new(applications)
        .block(Block::bordered().title("Application List:"))
        .highlight_symbol("->");

    StatefulWidget::render(
        l_widget,
        frame.area(),
        frame.buffer_mut(),
        &mut app.application_list_state,
    );
}
