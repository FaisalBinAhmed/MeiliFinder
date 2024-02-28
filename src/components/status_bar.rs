use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::Paragraph};

use crate::{app::App, Frame};

use super::input_bar;

pub fn draw_status_bar(f: &mut Frame, chunk: Rect, app: &App) {
    
        let app_mode_indicator: Vec<Span> = match app.app_mode {
        crate::app::AppMode::Normal => {
            vec![
            Span::styled(format!(" NORMAL "), Style::default().bg(Color::Blue).bold()),
            Span::styled(
            format!(" <q> close app | <tab> switch tabs | <enter> de/select document | <r> reload results | <s> search mode "),
            Style::default()),
            // todo: move this to its own chunk and align it on the far right side
            // Span::styled(
            // format!("Last refreshed: {}", &app.last_refreshed),
            // Style::default().fg(Color::LightCyan))
            ]
        }
        crate::app::AppMode::Search => {
            vec![
                Span::styled(format!(" SEARCH "), Style::default().bg(Color::Red).bold()),
                Span::styled(
                    format!(
                        " <esc> back to normal mode | <tab> navigate forms | <enter> submit search "
                    ),
                    Style::default(),
                ),
            ]
        }
    };


    let status_bar = Line::from(app_mode_indicator);

    // first part goes to the input field
    let bottom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(chunk);

    // input bar
    if app.app_mode == crate::app::AppMode::Search {
        input_bar::draw_input_bar(f, bottom_chunks[0], app);
    }

    f.render_widget(Paragraph::new(status_bar).bg(Color::Rgb(54, 54, 54)), bottom_chunks[1]);

}