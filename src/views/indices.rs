
use ratatui::{layout::Rect, style::Style, widgets::{Block, Borders, Clear, Padding}};

use crate::{app::App, Frame};

use crate::components::static_widgets;

pub fn draw_indices(f: &mut Frame, chunk: Rect,  app: &App) {

    let popup_title = "Indices";

    let block = Block::default()
        .title(popup_title)
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1))
        .style(Style::default());

    let area = static_widgets::centered_rect(69, 50, f.size());
    f.render_widget(Clear, area); //this clears out the background

}