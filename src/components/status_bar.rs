use ratatui::{prelude::*, widgets::Paragraph};

use crate::{app::App, Frame};

use super::input_bar;

pub fn draw_status_bar(f: &mut Frame, chunk: Rect, app: &App) {
    
        let app_mode_indicator: Vec<Span> = match app.app_mode {
        crate::app::AppMode::Normal => {
            vec![
            Span::styled(format!(" NORMAL "), Style::default().fg(Color::Black).bg(Color::Blue).bold()),
            Span::styled(
            format!(" <q> close app | <tab> switch tabs | <enter> de/select document | <r> refresh | <s> search mode | <space> item acions "),
            Style::default()),
            ]
        }
        crate::app::AppMode::Search => {
            vec![
                Span::styled(format!(" SEARCH "), Style::default().fg(Color::Black).bg(Color::Red).bold()),
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
        .constraints([Constraint::Length(3), Constraint::Length(1)])
        .split(chunk);

    // input bar
    if app.app_mode == crate::app::AppMode::Search {
        input_bar::draw_input_bar(f, bottom_chunks[0], app);
    }

    let last_line_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(bottom_chunks[1]);

    f.render_widget(Paragraph::new(status_bar).bg(Color::Rgb(54, 54, 54)), last_line_chunks[0]);


    let last_refreshed = Line::from(vec![
        Span::styled(
        format!(" Refreshed: "),
        Style::default().fg(Color::White).bg(Color::Rgb(24, 24, 24))
        
        ),
        Span::styled(
            format!(" {} ", &app.last_refreshed),
            Style::default().fg(Color::Black).bold().bg(Color::Green),
        ),

    ]);

    f.render_widget(Paragraph::new(last_refreshed).alignment(Alignment::Right).bg(Color::Rgb(54, 54, 54)), last_line_chunks[1]);


}