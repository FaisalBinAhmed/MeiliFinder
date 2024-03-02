
use ratatui::{layout::Rect, style::Style, text::{Line, Span, Text}, widgets::{Block, Borders, Clear, List, ListItem, Padding}};

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
                ))
            
            ]
            )
        })
        .collect::<Vec<ListItem>>());


    let block = Block::default()
        .title(" Indices ")
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1))
        .style(Style::default());

    f.render_widget(indices_list.block(block), chunk);




}