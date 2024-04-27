use crossterm::event::{self, Event, KeyCode};
use std::{io::{self, Write}, fs::OpenOptions};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Tabs, Paragraph, Clear},
    Frame, Terminal,
};

use crate::reg_gen::register::*;
use crate::reg_gen::json_handling::*;
use crate::vertical_tab::*;

// This file is responsible for all TUI operations. App stores the state of the TUI, and run_app
// runs the TUI

const BG_COLOR: Color = Color::Rgb(40, 44, 52);

pub enum AppState {
    SelectRegisterAndField,
    SelectFieldInfo,
    EditFieldInfo,
    AddRegister,
}

pub struct App {
    pub original_path: String,
    pub register_family: RegisterFamily,
    pub register_index: usize,
    pub register_info_index: usize,
    pub field_index: usize,
    pub field_info_index: usize,
    pub state: AppState,
    pub input: String,
}

impl App {
    pub fn new(path: String) -> App {
        App {
            original_path: path.clone(),
            register_family: pull_existing_json(&path),
            register_index: 0,
            register_info_index: 0,
            field_index: 0,
            field_info_index: 0,
            state: AppState::SelectRegisterAndField,
            input: String::new(),
        }
    }

    pub fn next_register(&mut self) {
        // Reset field index
        self.field_index = 0;
        self.field_info_index = 0;

        self.register_index = (self.register_index + 1) % self.register_family.registers.len();
    }

    pub fn previous_register(&mut self) {
        // Reset field index
        self.field_index = 0;
        self.field_info_index = 0;

        if self.register_index > 0 {
            self.register_index -= 1;
        } else {
            self.register_index = self.register_family.registers.len() - 1;
        }
    }

    pub fn next_field(&mut self) {
        self.field_info_index = 0;

        self.field_index = (self.field_index + 1) % self.register_family.registers[self.register_index].fields.len();
    }

    pub fn previous_field(&mut self) {
        self.field_info_index = 0;

        if self.field_index > 0 {
            self.field_index -= 1;
        } else {
            self.field_index = self.register_family.registers[self.register_index].fields.len() - 1;
        }
    }

    pub fn next_field_info(&mut self) {
        // Mod 5 because there are 5 field info elements
        self.field_info_index = (self.field_info_index + 1) % 5;
    }

    pub fn previous_field_info(&mut self) {
        if self.field_info_index > 0 {
            self.field_info_index -= 1;
        } else {
            // 4 because there are 5 field info elements
            self.field_info_index = 4;
        }
    }

    pub fn next_register_info(&mut self) {
        // Mod 2 because there are 2 info elements for a register
        self.register_info_index = (self.register_info_index + 1) % 2;
    }

    pub fn previous_register_info(&mut self) {
        if self.register_info_index > 0 {
            self.register_info_index -= 1;
        } else {
            self.register_info_index = 1;
        }
    }

    pub fn set_field_info(&mut self) {
        let field = &mut self.register_family.registers[self.register_index].fields[self.field_index];
        match self.field_info_index {
            0 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.lsb = x;
                }
            },
            1 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.msb = x;
                }
            },
            2 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.read = x;
                }
            },
            3 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.write = x;
                }
            },
            4 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.negative = Some(x);
                }
            },
            _ => (),
        }
    }
    pub fn write_to_file(&mut self) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .truncate(true)
            .open(self.original_path.clone())
            .expect("Could not open JSON file!");

        return match file.write_all(serde_json::to_string_pretty(&self.register_family).unwrap().as_bytes()) {
            Ok(_) => Ok(()),
            Err(why) => Err(std::io::Error::new(why.kind(), format!("Couldn't write to {}: {}", self.original_path, why))),
        };
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::SelectRegisterAndField => {
                    match key.code {
                        KeyCode::Char('q') => return app.write_to_file(),
                        KeyCode::Right => app.next_register(),
                        KeyCode::Left => app.previous_register(),
                        KeyCode::Up => app.previous_field(),
                        KeyCode::Down => app.next_field(),
                        KeyCode::Enter => app.state = AppState::SelectFieldInfo,
                        KeyCode::Char('a') => app.state = AppState::AddRegister,
                        _ => ()
                    }
                },
                AppState::AddRegister => {
                    match key.code {
                        KeyCode::Char(ch) => {
                            app.input.push(ch);
                        },
                        KeyCode::Backspace => {
                            app.input.pop();
                        },
                        KeyCode::Up => {
                            app.previous_register_info();
                        },
                        KeyCode::Down => {
                            app.next_register_info();
                        },
                        KeyCode::Esc => app.state = AppState::SelectRegisterAndField,
                        _ => ()
                    }
                },
                AppState::SelectFieldInfo => {
                    match key.code {
                        KeyCode::Char('q') => return app.write_to_file(),
                        KeyCode::Up => app.previous_field_info(),
                        KeyCode::Down => app.next_field_info(),
                        KeyCode::Enter => app.state = AppState::EditFieldInfo,
                        KeyCode::Esc => app.state = AppState::SelectRegisterAndField,
                        _ => ()
                    }
                },
                AppState::EditFieldInfo => {
                    match key.code {
                        KeyCode::Char(ch) => {
                            app.input.push(ch);
                        },
                        KeyCode::Backspace => {
                            app.input.pop();
                        },
                        KeyCode::Enter => {
                            app.set_field_info();
                        },
                        KeyCode::Esc => {
                            app.input.clear();
                            app.state = AppState::SelectFieldInfo;
                        },
                        _ => ()
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // Not sure what this does yet, look into in the future
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    // Full screen block I think
    let block = Block::default().style(Style::default().bg(BG_COLOR).fg(Color::White));
    f.render_widget(block, size);

    // We always draw these views no matter the app state
    draw_register_tabs(f, app, chunks[0]);
    draw_register_view(f, app, chunks[1]);
    
    match app.state {
        AppState::EditFieldInfo => draw_field_edit_popup(f, app, size),
        AppState::AddRegister => draw_register_add_popup(f, app, size),
        _ => (),
    }

    // Draw the input legend
    let mut text = Vec::new();
    match app.state {
        AppState::SelectRegisterAndField => {
            text.push(Spans::from("[Q]: Write and quit, [RIGHT]:Next register, [LEFT]: Previous register, [DOWN]: Next field, [UP]: Previous field, [ENTER]: Select field, [A]: Add register"));
        }
        AppState::SelectFieldInfo => {
            text.push(Spans::from("[Q]: Write and quit, [DOWN]: Next field info, [UP]: Previous field info, [ENTER]: Edit field info, [ESC]: Go back"));
        }
        AppState::EditFieldInfo => {
            text.push(Spans::from("[ENTER]: Set, [ESC]: Go back"));
        }
        AppState::AddRegister => {
            text.push(Spans::from("[DOWN]: Next field, [UP]: Previous field, [ENTER]: Create register, [ESC]: Go back"));
        }
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Controls").style(Style::default().bg(BG_COLOR)))
        .style(Style::default().bg(BG_COLOR).fg(Color::White));
    f.render_widget(Clear, chunks[2]);
    f.render_widget(paragraph, chunks[2]);
}

fn draw_register_tabs<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    // Create the tabs
    let titles = app
        .register_family
        .registers
        .iter()
        .map(|register| {
            Spans::from(register.name.clone())
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.register_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(if matches!(app.state, AppState::SelectRegisterAndField) { Color::LightMagenta } else { Color::DarkGray }),
        );
    f.render_widget(tabs, area);
}

fn draw_register_view<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    // Draw a box around the entire register view
    let block = Block::default()
        .style(Style::default()
            .bg(BG_COLOR)
            .fg(Color::White))
        .borders(Borders::ALL)
        .title("Register View");
    f.render_widget(block, area);

    // Vertical layout to specify register info at the top and fields at the bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(area);

    draw_register_information(f, app, chunks[0]);
    draw_field_view(f, app, chunks[1]);
}

fn draw_register_information<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    let text = vec![Spans::from(format!("Register Name: {}", app.register_family.registers[app.register_index].name)), Spans::from(format!("Size: {}-Bit", app.register_family.registers[app.register_index].size))];
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(BG_COLOR).fg(Color::White).add_modifier(Modifier::BOLD));

    f.render_widget(paragraph, area);
}

fn draw_field_view<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    let block = Block::default()
        .style(Style::default()
            .bg(BG_COLOR)
            .fg(Color::White))
        .borders(Borders::ALL)
        .title("Field View");

    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);

    draw_field_tabs(f, app, chunks[0]);
    draw_field_info_tabs(f, app, chunks[1]);
}

fn draw_field_tabs<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    // Create the tabs
    let titles = app
        .register_family
        .registers[app.register_index]
        .fields
        .iter()
        .map(|field| {
            Spans::from(field.name.clone())
        })
        .collect();
    let tabs = VerticalTabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Fields"))
        .select(app.field_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(if matches!(app.state, AppState::SelectRegisterAndField) { Color::LightMagenta } else { Color::DarkGray }),
        );
    f.render_widget(tabs, area);
}

fn draw_field_info_tabs<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    let field = &app.register_family.registers[app.register_index].fields[app.field_index];
    let titles = vec![
        Spans::from(format!("LSB: {}", field.lsb)),
        Spans::from(format!("MSB: {}", field.msb)),
        Spans::from(format!("Read: {}", field.read)),
        Spans::from(format!("Write: {}", field.write)),
        Spans::from(format!("Negative: {}", if let Some(n) = field.negative {n} else {false}))];
    let tabs = VerticalTabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Fields"))
        .select(app.field_info_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(if matches!(app.state, AppState::SelectFieldInfo) { Color::LightMagenta } else { Color::DarkGray }),
        );
    f.render_widget(tabs, area);
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
                .bg(if matches!(app.state, AppState::SelectFieldInfo) { Color::LightMagenta } else { Color::DarkGray }),
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

fn draw_field_edit_popup<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    let area = centered_rect(60, 20, area);
    let block = Block::default().title("Edit Field").borders(Borders::ALL);
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);

    // By making a vertical layout of 3 chunks with center chunk having defined length, we can
    // center text
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Ratio(1, 2), Constraint::Length(1), Constraint::Ratio(1, 2)].as_ref())
        .split(area);

    let (key, _value) = get_selected_field_as_string(app);
    draw_input_prompt(f, app, chunks[1], format!("Set {} to: ", key));
}

fn draw_input_prompt<B>(f: &mut Frame<B>, app: &mut App, area: Rect, prompt: String) where B: Backend {
    // Splitting into 4 chunks, chunk 0 is the prompt, chunk 1 is the input, chunk 2 is the cursor,
    // and chunk 3 pushes cursor to end of prompt
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Length(app.input.len() as u16), Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(area);

    // Draw the prompt
    let text = vec![Spans::from(prompt)];
    let paragraph = Paragraph::new(text)
        .alignment(tui::layout::Alignment::Right).style(Style::default().bg(BG_COLOR).fg(Color::White));
    f.render_widget(paragraph, chunks[0]);

    // Draw the current input
    let text = vec![Spans::from(app.input.clone())];
    let paragraph = Paragraph::new(text)
        .alignment(tui::layout::Alignment::Left).style(Style::default().bg(Color::Magenta).fg(Color::White));
    f.render_widget(paragraph, chunks[1]);

    // Draw the cursor
    let cursor_block = Block::default().style(Style::default().bg(Color::White));
    f.render_widget(cursor_block, chunks[2]);
}

fn get_selected_field_as_string(app: &App) -> (String, String) {
    let field = &app.register_family.registers[app.register_index].fields[app.field_index];
    match app.field_info_index {
        0 => (String::from("LSB"), field.lsb.to_string()),
        1 => (String::from("MSB"), field.msb.to_string()),
        2 => (String::from("Read"), field.read.to_string()),
        3 => (String::from("Write"), field.write.to_string()),
        4 => (String::from("Negative"), match field.negative { Some(x) => { x.to_string() }, None => { false.to_string() } }),
        _ => (String::from("ERROR"), String::from("ERROR")),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
