use ratatui::{layout::Rect, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{List, ListItem, Padding, Wrap}};

use crate::{app::App, utilities::helpers::get_task_type_name, Frame};

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
                        Span::styled(format!(" {} ", content.uid), Style::default()),
                        Span::styled(
                            format!("{} ", get_task_type_name(&content.update_type) ),
                            Style::default().bold(),
                        ),
                        Span::styled(" Enqueued ", Style::default().fg(Color::Yellow)),
                    ]),
                    
                        Line::from(vec![
                            Span::styled(
                            format!(" Enqueued at: {} ", content.enqueued_at.time()),
                            Style::default(),
                        )])
                    
                ])
                ,
                meilisearch_sdk::Task::Processing { content }=> 
                     ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!(" {} ", content.uid), Style::default()),
                        Span::styled(
                            format!("{} ", get_task_type_name(&content.update_type) ),
                            Style::default().bold(),
                        ),
                        Span::styled(" Processing ", Style::default().fg(Color::Cyan)),
                    ]),
                    

                    Line::from(vec![
                        Span::styled(
                            format!(" Started at: {} ", content.started_at.time()),
                            Style::default(),
                        ),

                ])



                ])
                ,
                meilisearch_sdk::Task::Failed { content }=> 
                     ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!(" {} ", content.task.uid), Style::default()),
                        Span::styled(
                            format!("{} ", get_task_type_name(&content.task.update_type)),
                            Style::default().bold(),
                        ),
                        Span::styled(" Failed ", Style::default().fg(Color::Red)),
                    ]),
                    

                    Line::from(vec![
                        Span::styled(
                            format!(" Finished at: {} ms", content.task.finished_at.time()),
                            Style::default(),
                        ),
                        Span::styled(
                            format!(" Error: {} ", content.error.error_message),
                            Style::default(),
                        ),

                ])



                ])
                ,
                meilisearch_sdk::Task::Succeeded { content }=> {
                     ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(format!(" {} ", content.uid), Style::default()),
                        Span::styled(
                            format!("{} ", get_task_type_name(&content.update_type) ),
                            Style::default().bold(),
                        ),
                        Span::styled(" Succeeded ", Style::default().fg(Color::Green)),
                    ]),
                        Line::from(vec![
                        Span::styled(
                            format!(" Duration: {} ms", content.duration.as_millis()),
                            Style::default(),
                        ),
                        Span::styled(
                            format!(" Finished at: {} ", content.finished_at.time()),
                            Style::default(),
                        ),

                ])



                ])
                },
            };

            list_item


        })
        .collect::<Vec<ListItem>>())
        .block(ratatui::widgets::Block::default()
            .title(" Tasks ")
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .fg(Color::DarkGray))
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .highlight_style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED));


    // f.render_widget(list, chunk);
    let list_state = &mut app.task_scroll_state.clone();
    f.render_stateful_widget(list, horizontal_chunks[0], list_state);

    let task_info = ratatui::widgets::Paragraph::new(format!("{}", app.get_current_task_info()))
        .block(ratatui::widgets::Block::default()
            .title(" Task Info ")
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::uniform(1))
            .fg(Color::DarkGray))
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .wrap(Wrap { trim: false });


    f.render_widget(task_info, horizontal_chunks[1]);

}