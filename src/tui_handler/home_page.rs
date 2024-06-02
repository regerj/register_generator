use tui::{Frame, layout::{Rect, Layout, Direction, Constraint}, backend::Backend, widgets::{Block, Paragraph, Borders, Clear, Tabs}, style::{Style, Color, Modifier}, text::Spans};

use crate::vertical_tab::VerticalTabs;

use super::{tui_root_handler::BG_COLOR, util::{centered_rect, draw_input_prompt}};
use crate::tui_handler::app_state::App;

#[derive(Clone)]
pub enum HomePageState {
    SelectRegisterAndField,
    SelectFieldInfo,
    EditFieldInfo,
}

pub struct HomePage<'a, B> where B: Backend {
    pub state: HomePageState,
    pub frame: &'a mut Frame<'a, B>,
    pub app: &'a mut App,
}

impl <B> HomePage<'_, B> where B: Backend {
    pub fn draw(&mut self, area: Rect) {
        // Setup the layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(3)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);

        // Full screen block I think
        let block = Block::default().style(Style::default().bg(BG_COLOR).fg(Color::White));
        self.frame.render_widget(block, area);

        // We always draw these views no matter the app state
        self.draw_register_tabs(chunks[0]);
        self.draw_register_view(chunks[1]);
        
        // Draw the popup if necessary
        if matches!(self.state, HomePageState::EditFieldInfo) {
            self.draw_field_edit_popup(area);
        }

        // Draw the input legend
        let mut text = Vec::new();
        match self.state {
            HomePageState::SelectRegisterAndField => {
                text.push(Spans::from("[Q]: Write and quit, [RIGHT]:Next register, [LEFT]: Previous register, [DOWN]: Next field, [UP]: Previous field, [ENTER]: Select field, [A]: Add register"));
            }
            HomePageState::SelectFieldInfo => {
                text.push(Spans::from("[Q]: Write and quit, [DOWN]: Next field info, [UP]: Previous field info, [ENTER]: Edit field info, [ESC]: Go back"));
            }
            HomePageState::EditFieldInfo => {
                text.push(Spans::from("[ENTER]: Set, [ESC]: Go back"));
            }
            // HomePageState::AddRegister => {
            //     text.push(Spans::from("[DOWN]: Next field, [UP]: Previous field, [ENTER]: Create register, [ESC]: Go back"));
            // }
        }

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Controls").style(Style::default().bg(BG_COLOR)))
            .style(Style::default().bg(BG_COLOR).fg(Color::White));
        self.frame.render_widget(Clear, chunks[2]);
        self.frame.render_widget(paragraph, chunks[2]);
    }

    fn draw_register_tabs(&mut self, area: Rect) {
        // Create the tabs
        let titles = self.app
            .register_family
            .registers
            .iter()
            .map(|register| {
                Spans::from(register.name.clone())
            })
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.app.register_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(if matches!(self.state, HomePageState::SelectRegisterAndField) { Color::LightMagenta } else { Color::DarkGray }),
            );
        self.frame.render_widget(tabs, area);
    }

    fn draw_register_view(&mut self, area: Rect) {
        // Draw a box around the entire register view
        let block = Block::default()
            .style(Style::default()
                .bg(BG_COLOR)
                .fg(Color::White))
            .borders(Borders::ALL)
            .title("Register View");
        self.frame.render_widget(block, area);

        // Vertical layout to specify register info at the top and fields at the bottom
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
            .split(area);

        self.draw_register_information(chunks[0]);
        self.draw_field_view(chunks[1]);
    }

    fn draw_field_edit_popup(&mut self, area: Rect) where B: Backend {
        let area = centered_rect(60, 20, area);
        let block = Block::default().title("Edit Field").borders(Borders::ALL);
        self.frame.render_widget(Clear, area); //this clears out the background
        self.frame.render_widget(block, area);

        // By making a vertical layout of 3 chunks with center chunk having defined length, we can
        // center text
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Ratio(1, 2), Constraint::Length(1), Constraint::Ratio(1, 2)].as_ref())
            .split(area);

        let (key, _value) = self.app.get_selected_field_as_string();
        draw_input_prompt(self.frame, &mut self.app.input, chunks[1], format!("Set {} to: ", key));
    }

    fn draw_register_information(&mut self, area: Rect) {
        let text = vec![Spans::from(format!("Register Name: {}", self.app.register_family.registers[self.app.register_index].name)), Spans::from(format!("Size: {}-Bit", self.app.register_family.registers[self.app.register_index].size))];
        let paragraph = Paragraph::new(text.clone())
            .style(Style::default().bg(BG_COLOR).fg(Color::White).add_modifier(Modifier::BOLD));

        self.frame.render_widget(paragraph, area);
    }

    fn draw_field_view(&mut self, area: Rect) {
        let block = Block::default()
            .style(Style::default()
                .bg(BG_COLOR)
                .fg(Color::White))
            .borders(Borders::ALL)
            .title("Field View");

        self.frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        self.draw_field_tabs(chunks[0]);
        self.draw_field_info_tabs(chunks[1]);
    }

    fn draw_field_tabs(&mut self, area: Rect) {
        // Create the tabs
        let titles = self.app
            .register_family
            .registers[self.app.register_index]
            .fields
            .iter()
            .map(|field| {
                Spans::from(field.name.clone())
            })
            .collect();
        let tabs = VerticalTabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Fields"))
            .select(self.app.field_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(if matches!(self.state, HomePageState::SelectRegisterAndField) { Color::LightMagenta } else { Color::DarkGray }),
            );
        self.frame.render_widget(tabs, area);
    }

    fn draw_field_info_tabs(&mut self, area: Rect) {
        let field = &self.app.register_family.registers[self.app.register_index].fields[self.app.field_index];
        let titles = vec![
            Spans::from(format!("LSB: {}", field.lsb)),
            Spans::from(format!("MSB: {}", field.msb)),
            Spans::from(format!("Read: {}", field.read)),
            Spans::from(format!("Write: {}", field.write)),
            Spans::from(format!("Negative: {}", if let Some(n) = field.negative {n} else {false}))];
        let tabs = VerticalTabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Fields"))
            .select(self.app.field_info_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(if matches!(self.state, HomePageState::SelectFieldInfo) { Color::LightMagenta } else { Color::DarkGray }),
            );
        self.frame.render_widget(tabs, area);
    }
}
