use std::rc::Rc;

use crate::{App, app::TriggerAction, app_selection::AppSelectionStatus};
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
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(10)])
        .flex(Flex::Center)
        .split(frame.area());

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
        .map(|(_i, application)| {
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

pub fn render_website_list(frame: &mut Frame, app: &mut App) {}

pub fn render_watching_screen(frame: &mut Frame, app: &mut App) {
    frame.render_widget(Block::bordered(), frame.area());

    let layout = Layout::vertical([Constraint::Length(20), Constraint::Min(10)])
        .flex(Flex::Center)
        .split(frame.area());

    let app_list_items: Vec<ListItem> = app
        .confirmed_app_list
        .applications
        .iter()
        .enumerate()
        .map(|(_i, a)| ListItem::from(a.name.to_string()))
        .collect();

    //TODO:
    //This text should change depending on what trigger action the user has chosen.
    //Add text for all options later on.
    let info_par = match app.trigger_action {
        TriggerAction::Shutdown => Paragraph::new(
            "The following apps will cause the computer to shut down if they are open/opened while GetOff is running:",
        ),
        TriggerAction::Warn => Paragraph::new(
            "The following apps will result in a warning if they are open/opened while GetOff is running:",
        ),
        TriggerAction::Restart => Paragraph::new(
            "The following apps will cause the computer to restart if they are open/opened while GetOff is running:",
        ),
        TriggerAction::Close => Paragraph::new(
            "The following apps will close if they are open/opened while GetOff is running:",
        ),
    };

    let app_list = List::new(app_list_items);

    frame.render_widget(info_par, layout[0]);
    frame.render_widget(app_list, layout[1]);
}
