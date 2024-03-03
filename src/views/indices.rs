
use ratatui::{layout::Rect, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Padding}};

use crate::{app::App, Frame};


pub fn draw_indices(f: &mut Frame, chunk: Rect,  app: &App) {

    let indices_list: List = List::new(app.indices
        .iter()
        .map(|index| {
            ListItem::new(vec![
                Line::from(Span::styled(
                    format!(" {} ", index.uid),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" {} ", index.primary_key.clone().unwrap_or("No primary key found".to_string() ) ),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" {} ", index.created_at.unwrap().time()),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" {} ", index.updated_at.unwrap().time()),
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
        .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().fg(Color::DarkGray));

    let list_state = &mut app.indices_scroll_state.clone();

    f.render_stateful_widget(indices_list.block(block), chunk, list_state);




}