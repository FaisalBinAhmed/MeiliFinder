use ratatui::{prelude::*, widgets::Paragraph};

use crate::{app::App, Frame};

use super::input_bar;

pub fn draw_status_bar(f: &mut Frame, chunk: Rect, app: &App) {
    
        let app_mode_indicator: Vec<Span> = match app.app_mode {
        crate::app::AppMode::Normal => {
            vec![
            Span::styled(format!(" NORMAL "), Style::default().fg(Color::Rgb(0, 0, 0)).bg(Color::Blue).bold()),
            Span::styled(
            format!(" <q> close app | <tab> switch tabs | <r> refresh | <s> search mode | <space> item actions "),
            Style::default()),
            ]
        }
        crate::app::AppMode::Search => {
            vec![
                Span::styled(format!(" SEARCH "), Style::default().fg(Color::Rgb(0, 0, 0)).bg(Color::Rgb(255, 205, 170)).bold()),
                Span::styled(
                    format!(
                        " <esc> back to normal mode | <tab> navigate queries | <enter> submit search | <> clear all "
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
            Style::default().fg(Color::Rgb(0, 0, 0)).bold().bg(Color::Rgb(131, 118, 156)),
        ),

    ]);

    f.render_widget(Paragraph::new(last_refreshed).alignment(Alignment::Right).bg(Color::Rgb(54, 54, 54)), last_line_chunks[1]);


}