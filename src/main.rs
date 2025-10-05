use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, Terminal,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;

// Struct that represents the state of the app.
#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    // Responsible for core loop of drawing frames for the application.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // continue drawing frames until exit flag is true
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // draw a frame using render_widget method
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    // function to handle events, currently only handles key presses.
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    // handle key presses by the user,
    // for now treats all key presses the same way by exiting.
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(_) => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = Line::from(" GET OFF! ".bold().italic());
        let block = Block::bordered()
            .title(header.centered())
            .border_set(border::DOUBLE);

        // render everything with a paragraph
        Paragraph::new("This getoff app sure will be cool once it's done!")
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    // initialize a new DefaultTerminal
    let mut terminal = ratatui::init();

    // Start core application cycle
    let app_result = App::default().run(&mut terminal);

    // restore terminal to original state and resurn
    ratatui::restore();
    app_result
}
