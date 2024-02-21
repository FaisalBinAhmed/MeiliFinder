use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Tabs},
};

use crate::{
    app::{App, AppTabs},
    // tui::Frame,
};

pub fn render(app: &mut App, f: &mut ratatui::Frame) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let titles: Vec<Line> = ["Departures", "Station List"]
        .iter()
        .map(|t| {
            Line::from(Span::styled(
                format!("{}", t),
                Style::default().fg(Color::LightCyan),
            ))
        })
        .collect();
    let index: usize = match app.selected_tab {
        AppTabs::HomeTab => 0,
        AppTabs::StationTab => 1,
    };


    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" MVG FahrInfo "),
        )
        .select(index)
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green));

    f.render_widget(tabs, chunks[0]);


    //Status bar


    // let status_bar = Line::from(app_mode_indicator);

    // f.render_widget(Paragraph::new(status_bar), chunks[2]);

    //SEARCH MODAL
    //todo: move to its own component

    if app.app_mode == crate::app::AppMode::Search {
        let popup_title = " ⌕ Search for a station ";

        // let mut text = Text::from(Line::from(app.query.clone()));
        // text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

        // let input_field = Paragraph::new(text)
        //     .block(Block::default().borders(Borders::ALL).title(popup_title))
        //     .style(Style::default().fg(Color::LightCyan))
        //     .alignment(ratatui::prelude::Alignment::Left);

        // let area = static_widgets::centered_rect(69, 50, f.size()); //size of the MODAL

        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints([Constraint::Length(3), Constraint::Min(0)])
        //     .split(area);

        // f.render_widget(Clear, area); //this clears out the background
        // f.render_widget(input_field, chunks[0]);
        // f.set_cursor(
        //     // Draw the cursor at the current position in the input field.
        //     // This position is can be controlled via the left and right arrow key
        //     chunks[0].x + app.cursor_position as u16 + 1,
        //     // Move one line down, from the border to the input line
        //     chunks[0].y + 1,
        // );

        //search suggestion section
    }
}

