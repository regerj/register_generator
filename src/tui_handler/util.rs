use tui::{layout::{Rect, Layout, Direction, Constraint}, Frame, backend::Backend, text::Spans, widgets::{Paragraph, Block}, style::{Style, Color}};

use super::tui_root_handler::BG_COLOR;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn draw_input_prompt<B>(f: &mut Frame<B>, user_input: &mut String, area: Rect, prompt: String) where B: Backend {
    // Splitting into 4 chunks, chunk 0 is the prompt, chunk 1 is the input, chunk 2 is the cursor,
    // and chunk 3 pushes cursor to end of prompt
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Length(user_input.len() as u16), Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(area);

    // Draw the prompt
    let text = vec![Spans::from(prompt)];
    let paragraph = Paragraph::new(text)
        .alignment(tui::layout::Alignment::Right).style(Style::default().bg(BG_COLOR).fg(Color::White));
    f.render_widget(paragraph, chunks[0]);

    // Draw the current input
    let text = vec![Spans::from(user_input.clone())];
    let paragraph = Paragraph::new(text)
        .alignment(tui::layout::Alignment::Left).style(Style::default().bg(Color::Magenta).fg(Color::White));
    f.render_widget(paragraph, chunks[1]);

    // Draw the cursor
    let cursor_block = Block::default().style(Style::default().bg(Color::White));
    f.render_widget(cursor_block, chunks[2]);
}
