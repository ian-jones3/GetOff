use std::rc::Rc;

use crate::{App, app_selection::AppSelectionStatus};
use ratatui::{
    Frame,
    layout::*,
    style::{Style, Styled},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, StatefulWidget},
};
use tui_widgets::prompts::{Prompt, State, TextPrompt};

//TODO:
//Redo styling to be in line with other screens
//1. Border around entire screen
//2. Text prompt is bordered, prompt text is title in border block
pub fn render_application_list(frame: &mut Frame, app: &mut App) {
    let layout = build_layout(frame);

    // render search bar
    TextPrompt::from("Search for application:").draw(
        frame,
        layout[0],
        &mut app.app_list_search_state,
    );

    // unwrap is ok here, if app list building fails something is
    // fundamentally wrong.

    let applications: Vec<ListItem> = app
        // .application_list
        // .applications
        .filtered_app_list
        .application_tuples
        .iter()
        .enumerate()
        // Create ListItems from the applications, adding custom styling which highlights
        // whether or not they are currently selected.
        // Check whether the App is in the current search before adding to the list,
        // and check the master list to see if it should be highlighted as selected.
        .map(|(i, application)| {
            //ListItem::from(application.name.to_string()).set_style(match application.status {
            ListItem::from(application.0.name.to_string()).set_style(
                match app.application_list.applications[application.1].status {
                    AppSelectionStatus::Selected => Style::new().bg(ratatui::style::Color::Magenta),
                    AppSelectionStatus::NotSelected => {
                        Style::new().bg(ratatui::style::Color::Reset)
                    }
                },
            )
        })
        .collect();

    let l_widget = List::new(applications)
        .block(Block::bordered().title("Application List:"))
        .highlight_symbol("->");

    // render list
    StatefulWidget::render(
        l_widget,
        layout[1],
        frame.buffer_mut(),
        &mut app.application_list_state,
    );
}

fn build_layout(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::vertical([Constraint::Length(1), Constraint::Min(10)])
        .flex(Flex::Center)
        .split(frame.area())
}
