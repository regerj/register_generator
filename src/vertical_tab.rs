use tui::buffer::*;
use tui::text::*;
use tui::widgets::*;
use tui::style::*;
use tui::layout::*;

// My custom widget for vertical tabs since TUI doesn't support them natively for some reason

#[derive(Debug, Clone)]
pub struct VerticalTabs<'a> {
    /// A block to wrap this widget in if necessary
    block: Option<Block<'a>>,
    /// One title for each tab
    titles: Vec<Spans<'a>>,
    /// The index of the selected tabs
    selected: usize,
    /// The style used to draw the text
    style: Style,
    /// Style to apply to the selected item
    highlight_style: Style,
    // Tab divider
    // divider: Span<'a>,
}

impl<'a> VerticalTabs<'a> {
    pub fn new(titles: Vec<Spans<'a>>) -> VerticalTabs<'a> {
        VerticalTabs {
            block: None,
            titles,
            selected: 0,
            style: Default::default(),
            highlight_style: Default::default(),
            // divider: Span::raw(line::HORIZONTAL),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> VerticalTabs<'a> {
        self.block = Some(block);
        self
    }

    pub fn select(mut self, selected: usize) -> VerticalTabs<'a> {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: Style) -> VerticalTabs<'a> {
        self.style = style;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> VerticalTabs<'a> {
        self.highlight_style = style;
        self
    }

    // pub fn divider<T>(mut self, divider: T) -> VerticalTabs<'a>
    // where
    //     T: Into<Span<'a>>,
    // {
    //     self.divider = divider.into();
    //     self
    // }
}

impl<'a> Widget for VerticalTabs<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let tabs_area = match self.block.take() {
            Some(b) => {
                let mut inner_area = b.inner(area);
                b.render(area, buf);
                // I do this to align it better with the box
                inner_area.y -= 1;
                inner_area
            }
            None => area,
        };

        if tabs_area.height < 1 {
            return;
        }

        let mut y = tabs_area.top();
        let titles_length = self.titles.len();
        for (i, title) in self.titles.into_iter().enumerate() {
            let last_title = titles_length - 1 == i;
            y = y.saturating_add(1);
            let remaining_height = tabs_area.bottom().saturating_sub(y);
            if remaining_height == 0 {
                break;
            }
            let pos = buf.set_spans(tabs_area.left(), y, &title, remaining_height);
            if i == self.selected {
                buf.set_style(
                    Rect {
                        x: tabs_area.left(),
                        y,
                        width: title.width() as u16,
                        height: 1,
                    },
                    self.highlight_style,
                );
            }
            y = pos.1.saturating_add(1);
            let remaining_height = tabs_area.bottom().saturating_sub(y);
            if remaining_height == 0 || last_title {
                break;
            }
            // let pos = buf.set_span(tabs_area.left(), y, &self.divider, remaining_height);
            y = pos.1;
        }
    }
}
