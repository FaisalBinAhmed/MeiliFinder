use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{app::App, constants::INSTANCE_COLOR, Frame};

pub fn draw_instances(f: &mut Frame, chunk: Rect, app: &App) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(chunk);

    let current_instance: String = match &app.current_instance {
        Some(instance) => instance.name.clone(),
        None => "No instance selected".to_string(),
    };

    let instance_info = Line::from(vec![
        Span::styled(format!(" Selected instance: "), Style::default()),
        Span::styled(
            format!("{} ", current_instance),
            Style::default().fg(INSTANCE_COLOR).bold(),
        ),
    ]);

    let key_info_line = Line::from(vec![Span::styled(
        format!(" Press <enter> to change to selected instance "),
        Style::default(),
    )]);

    f.render_widget(
        Paragraph::new(vec![instance_info, key_info_line]).alignment(Alignment::Left),
        vertical_chunks[0],
    );

    let instances_list: List = List::new(
        app.instances
            .iter()
            .map(|instance| {
                ListItem::new(vec![
                    Line::from(Span::styled(
                        format!(" Id: {} ", instance.id),
                        Style::default(),
                    )),
                    Line::from(Span::styled(
                        format!(" Name: {} ", instance.name),
                        Style::default(),
                    )),
                    Line::from(Span::styled(
                        format!(" Host URL: {} ", instance.host),
                        Style::default(),
                    )),
                    Line::from(Span::styled(
                        format!(
                            " Primary key: {} ",
                            instance.primary_key.replace(|_| true, "*")
                        ),
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
        .title(" Instances ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        // .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().fg(Color::DarkGray));

    let list_state = &mut app.instances_scroll_state.clone();

    f.render_stateful_widget(instances_list.block(block), vertical_chunks[1], list_state);
}
