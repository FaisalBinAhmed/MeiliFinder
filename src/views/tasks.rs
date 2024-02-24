use ratatui::{layout::Rect, style::{Color, Style}, text::{Line, Span}, widgets::{List, ListItem, Wrap}};

use crate::{app::App, Frame};

pub fn draw_tasks(f: &mut Frame,  chunk: Rect, app: &App){


    let horizontal_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([ratatui::layout::Constraint::Percentage(70), ratatui::layout::Constraint::Percentage(30)])
        .split(chunk);

    let list: List = List::new(app.tasks
        .iter()
        .map(|t| {
            let list_item = match t {
                meilisearch_sdk::Task::Enqueued { content }=> 
                     ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!("{}", content.uid), Style::default()),
                        Span::styled(
                            format!("Enqueued at ({})", content.enqueued_at),
                            Style::default().fg(Color::LightCyan),
                        ),
                    ]),
                    // Line::from(get_product_icon_spans(&station.products)),
                ])
                ,
                meilisearch_sdk::Task::Processing { content }=> 
                     ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!("{}", content.uid), Style::default()),
                        Span::styled(
                            format!("Started at ({})", content.started_at),
                            Style::default().fg(Color::LightCyan),
                        ),
                    ]),
                    // Line::from(get_product_icon_spans(&station.products)),
                ])
                ,
                meilisearch_sdk::Task::Failed { content }=> 
                     ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!("{}", content.task.uid), Style::default()),
                        Span::styled(
                            format!("Failed ({})", content.error),
                            Style::default().fg(Color::LightCyan),
                        ),
                    ]),
                    // Line::from(get_product_icon_spans(&station.products)),
                ])
                ,
                meilisearch_sdk::Task::Succeeded { content }=> {
                     ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!("{}", content.uid), Style::default()),
                        Span::styled(
                            format!("Finished at ({})", content.finished_at),
                            Style::default().fg(Color::LightCyan),
                        ),
                    ]),
                    // Line::from(get_product_icon_spans(&station.products)),
                ])
                },
            };

            return list_item;


        })
        .collect::<Vec<ListItem>>())
        .block(ratatui::widgets::Block::default().title(" Tasks ").borders(ratatui::widgets::Borders::ALL))
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .highlight_style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED));


    // f.render_widget(list, chunk);
    let list_state = &mut app.task_scroll_state.clone();
    f.render_stateful_widget(list, horizontal_chunks[0], list_state);


    // let task_text = app.tasks[0].clone();
    let task_text = app.current_task_info.clone().unwrap_or_else(|| String::from("No task selected"));

    let rrr= serde_json::to_string_pretty(&task_text).unwrap_or_else(|_| String::from("No task selected"));

    let task_info = ratatui::widgets::Paragraph::new(format!("{}", rrr))
        .block(ratatui::widgets::Block::default().title(" Task Info ").borders(ratatui::widgets::Borders::ALL))
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .wrap(Wrap { trim: true });
        // .alignment(ratatui::style::Alignment::Center);


    f.render_widget(task_info, horizontal_chunks[1]);








}