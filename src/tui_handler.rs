use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

use crate::reg_gen::register::*;
use crate::reg_gen::json_handling::*;

pub struct App {
    pub register_family: RegisterFamily,
    pub index: usize,
}

impl App {
    pub fn new(path: String) -> App {
        App {
            register_family: pull_existing_json(path),
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.register_family.registers.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.register_family.registers.len() - 1;
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
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
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Full screen block I think
    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    // Create the tabs
    let titles = app
        .register_family
        .registers
        .iter()
        .map(|register| {
            Spans::from(Span::styled(register.name.clone(), Style::default().fg(Color::Green)))
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    // Create register view
    // let inner = match app.index {
    //     0 => Block::default().title("Inner 0").borders(Borders::ALL),
    //     1 => Block::default().title("Inner 1").borders(Borders::ALL),
    //     2 => Block::default().title("Inner 2").borders(Borders::ALL),
    //     3 => Block::default().title("Inner 3").borders(Borders::ALL),
    //     _ => unreachable!(),
    // };
    draw_register_view(f, app, chunks[1]);
}

fn draw_register_view<B>(f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(area);
    // Block::default().title("EXAMPLE_TITLE").borders(Borders::ALL)
}
