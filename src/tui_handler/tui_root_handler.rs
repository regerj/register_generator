use crossterm::event::{self, Event, KeyCode};
use std::io::{self};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Clear},
    Frame, Terminal,
};

use crate::vertical_tab::VerticalTabs;

use super::{home_page::{HomePageState, HomePage}, util::centered_rect};
use crate::tui_handler::app_state::App;


// This file is responsible for all TUI operations. App stores the state of the TUI, and run_app
// runs the TUI

pub const BG_COLOR: Color = Color::Rgb(40, 44, 52);

pub enum BasePage {
    Home(HomePageState),
    AddRegister,
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        // Draw the TUI
        terminal.draw(|f| ui(f, &mut app))?;

        // Handle user inputs
        if let Event::Key(key) = event::read()? {
            match app.state {
                BasePage::Home(state) => match state {
                    HomePageState::SelectRegisterAndField => {
                        match key.code {
                            KeyCode::Char('q') => return app.write_to_file(),
                            KeyCode::Right => app.next_register(),
                            KeyCode::Left => app.previous_register(),
                            KeyCode::Up => app.previous_field(),
                            KeyCode::Down => app.next_field(),
                            KeyCode::Enter => app.state = BasePage::Home(HomePageState::SelectFieldInfo),
                            KeyCode::Char('a') => app.state = BasePage::AddRegister,
                            _ => ()
                        }
                    }
                    HomePageState::SelectFieldInfo => {
                        match key.code {
                            KeyCode::Char('q') => return app.write_to_file(),
                            KeyCode::Up => app.previous_field_info(),
                            KeyCode::Down => app.next_field_info(),
                            KeyCode::Enter => app.state = BasePage::Home(HomePageState::EditFieldInfo),
                            KeyCode::Esc => app.state = BasePage::Home(HomePageState::SelectRegisterAndField),
                            _ => ()
                        }
                    }
                    HomePageState::EditFieldInfo => {
                        match key.code {
                            KeyCode::Char(ch) => {
                                app.input.push(ch);
                            }
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            KeyCode::Enter => {
                                app.set_field_info();
                            }
                            KeyCode::Esc => {
                                app.input.clear();
                                app.state = BasePage::Home(HomePageState::SelectFieldInfo);
                            }
                            _ => ()
                        }
                    }
                }
                BasePage::AddRegister => match key.code {
                    KeyCode::Char(ch) => {
                        app.input.push(ch);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Up => {
                        app.previous_register_info();
                    }
                    KeyCode::Down => {
                        app.next_register_info();
                    }
                    KeyCode::Esc => app.state = BasePage::Home(HomePageState::SelectRegisterAndField),
                    _ => ()
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // Draw the page
    match &app.state {
        BasePage::Home(state) => {
            HomePage{
                state: state.clone(),
                frame: f,
                app
            }.draw(size);

            
        }
        BasePage::AddRegister => draw_add_register_page(f, app, size),
    }
}

fn draw_add_register_page<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {

}



fn draw_register_add_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    let area = centered_rect(60, 50, area);
    f.render_widget(Clear, area); //this clears out the background

    let titles = vec![
        Spans::from("Name: "),
        Spans::from("Size: ")];
    let tabs = VerticalTabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Add Register"))
        .select(app.field_info_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(if matches!(app.state, SelectFieldInfo) { Color::LightMagenta } else { Color::DarkGray }),
        );
    f.render_widget(tabs, area);

    // By making a vertical layout of 3 chunks with center chunk having defined length, we can
    // center text
    let _chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(area);
}
