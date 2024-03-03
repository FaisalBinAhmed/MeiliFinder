
use ratatui::{layout::Rect, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Padding}};

use crate::{app::App, Frame};


pub fn draw_instances(f: &mut Frame, chunk: Rect, app: &App) {


        let instances_list: List = List::new(app.instances
        .iter()
        .map(|instance| {
            ListItem::new(vec![
                Line::from(Span::styled(
                    format!(" {} ", instance.id),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" {} ", instance.name),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" {} ", instance.host),
                    Style::default()
                )),

                Line::from(Span::styled(
                    format!(" {} ", instance.primary_key),
                    Style::default()
                )),
            
            ]
            )
        })
        .collect::<Vec<ListItem>>())
        // .highlight_style(Style::default().bg(Color::Rgb(24, 24, 24)).fg(Color::White))
        // .style(Style::default().fg(Color::White));
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .highlight_style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED));

    let block = Block::default()
        .title(" Instances ")
        .borders(Borders::ALL)
        // .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().fg(Color::DarkGray));

    let list_state = &mut app.instances_scroll_state.clone();

    f.render_stateful_widget(instances_list.block(block), chunk, list_state);



}