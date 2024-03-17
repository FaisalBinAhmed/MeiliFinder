use ratatui::{prelude::*, widgets::Paragraph};

use crate::{
    app::app::{App, AppMode},
    constants::{DELETE_MODE_COLOR, PREVIEW_MODE_COLOR},
    Frame,
};

pub fn draw_status_bar(f: &mut Frame, chunk: Rect, app: &App) {
    let app_mode_indicator: Vec<Span> = match app.app_mode {
        AppMode::Normal => {
            vec![
            Span::styled(format!(" NORMAL "), Style::default().fg(Color::Rgb(0, 0, 0)).bg(Color::Blue).bold()),
            Span::styled(
            format!(" <q> exit app | <tab> switch tabs | <r> refresh | <s> search mode | <space> item actions | <ctrl + u> clear inputs "),
            Style::default()),
            // get_tab_specific_help(app.selected_tab)
            ]
        }
        AppMode::Search => {
            vec![
                Span::styled(format!(" SEARCH "), Style::default().fg(Color::Rgb(0, 0, 0)).bg(Color::Rgb(255, 205, 170)).bold()),
                Span::styled(
                    format!(
                        " <esc> back to normal mode | <tab> navigate queries | <enter> submit search "
                    ),
                    Style::default(),
                ),
            ]
        }
        AppMode::Preview => {
            vec![
                Span::styled(
                    format!(" PREVIEW "),
                    Style::default()
                        .fg(Color::Rgb(0, 0, 0))
                        .bg(PREVIEW_MODE_COLOR)
                        .bold(),
                ),
                Span::styled(
                    format!(" <esc> back to normal mode | <backspace> delete item "),
                    Style::default(),
                ),
            ]
        }
        AppMode::Delete => {
            vec![
                Span::styled(
                    format!(" DELETE "),
                    Style::default()
                        .fg(Color::Rgb(0, 0, 0))
                        .bg(DELETE_MODE_COLOR)
                        .bold(),
                ),
                Span::styled(
                    format!(" <esc> back to normal mode | <backspace> confirm delete "),
                    Style::default(),
                ),
            ]
        }
    };

    let status_bar = Line::from(app_mode_indicator);

    let last_line_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(chunk);

    f.render_widget(
        Paragraph::new(status_bar).bg(Color::Rgb(54, 54, 54)),
        last_line_chunks[0],
    );

    let last_refreshed = Line::from(vec![
        Span::styled(
            format!(" Refreshed: "),
            Style::default().fg(Color::White).bg(Color::Rgb(24, 24, 24)),
        ),
        Span::styled(
            format!(" {} ", &app.last_refreshed),
            Style::default()
                .fg(Color::Rgb(0, 0, 0))
                .bold()
                // .bg(Color::Rgb(131, 118, 156)),
                .bg(Color::LightMagenta)
        ),
    ]);

    f.render_widget(
        Paragraph::new(last_refreshed)
            .alignment(Alignment::Right)
            .bg(Color::Rgb(54, 54, 54)),
        last_line_chunks[1],
    );
}
