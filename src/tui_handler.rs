use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, Paragraph},
    Frame, Terminal,
};

use crate::reg_gen::register::*;
use crate::reg_gen::json_handling::*;
use crate::vertical_tab::*;

// This file is responsible for all TUI operations. App stores the state of the TUI, and run_app
// runs the TUI

const BG_COLOR: Color = Color::Rgb(40, 44, 52);

pub struct App {
    pub register_family: RegisterFamily,
    pub register_index: usize,
    pub field_index: usize,
    pub field_selected: bool,
}

impl App {
    pub fn new(path: String) -> App {
        App {
            register_family: pull_existing_json(path),
            register_index: 0,
            field_index: 0,
            field_selected: false,
        }
    }

    pub fn next_register(&mut self) {
        // Reset field index
        self.field_index = 0;

        self.register_index = (self.register_index + 1) % self.register_family.registers.len();
    }

    pub fn previous_register(&mut self) {
        // Reset field index
        self.field_index = 0;

        if self.register_index > 0 {
            self.register_index -= 1;
        } else {
            self.register_index = self.register_family.registers.len() - 1;
        }
    }

    pub fn next_field(&mut self) {
        self.field_index = (self.field_index + 1) % self.register_family.registers[self.register_index].fields.len();
    }

    pub fn previous_field(&mut self) {
        if self.field_index > 0 {
            self.field_index -= 1;
        } else {
            self.field_index = self.register_family.registers[self.register_index].fields.len() - 1;
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => {
                    if app.field_selected {
                        // Do nothing rn
                    } else {
                        app.next_register();
                    }
                },
                KeyCode::Left => {
                    if app.field_selected {
                        // Do nothing rn
                    } else {
                        app.previous_register();
                    }
                },
                KeyCode::Up => {
                    if app.field_selected {
                        // Do nothing rn
                    } else {
                        app.previous_field();
                    }
                },
                KeyCode::Down => {
                    if app.field_selected {
                        // Do nothing rn
                    } else {
                        app.next_field();
                    }
                },
                KeyCode::Enter => app.field_selected = true,
                KeyCode::Esc => app.field_selected = false,
                _ => {}
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
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Full screen block I think
    let block = Block::default().style(Style::default().bg(BG_COLOR).fg(Color::White));
    f.render_widget(block, size);

    draw_register_tabs(f, app, chunks[0]);
    draw_register_view(f, app, chunks[1]);
}

fn draw_register_tabs<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    // Create the tabs
    let titles = app
        .register_family
        .registers
        .iter()
        .map(|register| {
            Spans::from(Span::styled(register.name.clone(), Style::default().fg(Color::White).bg(Color::Magenta)))
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.register_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(BG_COLOR),
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
    draw_field_info(f, app, chunks[1]);
}

fn draw_field_tabs<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    // Create the tabs
    let titles = app
        .register_family
        .registers[app.register_index]
        .fields
        .iter()
        .map(|field| {
            Spans::from(Span::styled(field.name.clone(), Style::default().fg(Color::White).bg(Color::Magenta)))
        })
        .collect();
    let tabs = VerticalTabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Fields"))
        .select(app.field_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(BG_COLOR),
        );
    f.render_widget(tabs, area);
}

fn draw_field_info<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    let field = &app.register_family.registers[app.register_index].fields[app.field_index];
    let text = vec![
        Spans::from(format!("LSB: {}", field.lsb)),
        Spans::from(format!("MSB: {}", field.msb)),
        Spans::from(format!("Read: {}", field.read)),
        Spans::from(format!("Write: {}", field.write)),
        Spans::from(format!("Negative: {}", if let Some(n) = field.negative {n} else {false}))];
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(BG_COLOR).fg(Color::White)).block(Block::default().borders(Borders::ALL).title("Field Information"));

    f.render_widget(paragraph, area);
}
