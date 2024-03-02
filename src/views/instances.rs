
use ratatui::{style::Style, widgets::{Block, Borders, Clear, Padding}};

use crate::{app::App, Frame};

use crate::components::static_widgets;

pub fn draw_instances(f: &mut Frame, app: &App) {

    // let popup_title = "Instances";

    let area = static_widgets::centered_rect(69, 50, f.size());
    f.render_widget(Clear, area); //this clears out the background
    // f.render_widget(table, area);
}