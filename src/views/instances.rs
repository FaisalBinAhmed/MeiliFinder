
use ratatui::{style::Style, widgets::{Block, Borders, Clear, Padding}};

use crate::{app::App, Frame};

use crate::components::static_widgets;

pub fn draw_instances(f: &mut Frame, app: &App) {

    let popup_title = "Instances";

    let block = Block::default()
        .title(popup_title)
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1))
        .style(Style::default());

    // let table = display_departures_table(&app.departures).block(block);

    let area = static_widgets::centered_rect(69, 50, f.size());
    f.render_widget(Clear, area); //this clears out the background
    // f.render_widget(table, area);
}