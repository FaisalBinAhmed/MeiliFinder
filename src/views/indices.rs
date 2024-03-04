use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
};

use crate::{app::App, constants::INDEX_COLOR, Frame};

pub fn draw_indices(f: &mut Frame, chunk: Rect, app: &App) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(chunk);

    let current_index: String = match &app.current_index {
        Some(index) => index.uid.clone(),
        None => "No index selected".to_string(),
    };

    let index_info = Line::from(vec![
        Span::styled(format!(" Selected index: "), Style::default()),
        Span::styled(
            format!("{} ", current_index),
            Style::default().fg(INDEX_COLOR).bold(),
        ),
    ]);

    let key_info_line = Line::from(vec![
        Span::styled(
            format!(" Press <enter> to change to selected index "),
            Style::default(),
        ),
        // Span::styled(
        //     format!("{} ", current_index),
        //     Style::default().fg(INDEX_COLOR).bold(),
        // ),
    ]);

    f.render_widget(
        Paragraph::new(vec![index_info, key_info_line]).alignment(Alignment::Left),
        vertical_chunks[0],
    );

    let index_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(70),
            ratatui::layout::Constraint::Percentage(30),
        ])
        .split(vertical_chunks[1]);

    let indices_list: List = List::new(
        app.indices
            .iter()
            .map(|index| {
                ListItem::new(vec![
                    Line::from(Span::styled(
                        format!(" UID: {} ", index.uid),
                        Style::default(),
                    )),
                    Line::from(Span::styled(
                        format!(
                            " Primary key: {} ",
                            index
                                .primary_key
                                .clone()
                                .unwrap_or("No primary key found".to_string())
                        ),
                        Style::default(),
                    )),
                    Line::from(Span::styled(
                        format!(" Created at: {} ", index.created_at.unwrap().time()),
                        Style::default(),
                    )),
                    Line::from(Span::styled(
                        format!(" Updated at: {} ", index.updated_at.unwrap().time()),
                        Style::default(),
                    )),
                ])
            })
            .collect::<Vec<ListItem>>(),
    )
    // .highlight_style(Style::default().bg(Color::Rgb(24, 24, 24)).fg(Color::White))
    // .style(Style::default().fg(Color::White));
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
    .highlight_style(
        ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED),
    );

    let block = Block::default()
        .title(" Indices ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        // .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().fg(Color::DarkGray));

    let list_state = &mut app.indices_scroll_state.clone();

    f.render_stateful_widget(indices_list.block(block), index_chunks[0], list_state);

    // the second chunk is reserved for the index settings

    let index_settings = app.get_current_index_settings();

    let index_info = ratatui::widgets::Paragraph::new(format!("{}", index_settings))
        .block(
            ratatui::widgets::Block::default()
                .title(" Index Settings ")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .padding(Padding::uniform(1))
                .fg(Color::DarkGray),
        )
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(index_info, index_chunks[1]);
}
