
use ratatui::{layout::Rect, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Padding}};

use crate::{app::App, Frame};


pub fn draw_indices(f: &mut Frame, chunk: Rect,  app: &App) {

    let index_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([ratatui::layout::Constraint::Percentage(70), ratatui::layout::Constraint::Percentage(30)])
        .split(chunk);

    let indices_list: List = List::new(app.indices
        .iter()
        .map(|index| {
            ListItem::new(vec![
                Line::from(Span::styled(
                    format!(" UID: {} ", index.uid),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" Primary key: {} ", index.primary_key.clone().unwrap_or("No primary key found".to_string() ) ),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" Created at: {} ", index.created_at.unwrap().time()),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" Updated at: {} ", index.updated_at.unwrap().time()),
                    Style::default()
                )),
            
            ]
            )
        })
        .collect::<Vec<ListItem>>())
        .highlight_style(Style::default().bg(Color::Rgb(24, 24, 24)).fg(Color::White))
        .style(Style::default().fg(Color::White));


    let block = Block::default()
        .title(" Indices ")
        .borders(Borders::ALL)
        // .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().fg(Color::DarkGray));

    let list_state = &mut app.indices_scroll_state.clone();

    f.render_stateful_widget(indices_list.block(block), index_chunks[0], list_state);


    // the second chunk is reserved for the index settings

    let index_settings = app.get_current_index_settings();

    let index_info = ratatui::widgets::Paragraph::new(format!("{}", index_settings))
        .block(ratatui::widgets::Block::default().title(" Index Settings ").borders(ratatui::widgets::Borders::ALL).padding(Padding::uniform(1)).fg(Color::DarkGray))
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true });


    f.render_widget(index_info, index_chunks[1]);




}