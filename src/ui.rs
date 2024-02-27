use ratatui::{
    layout::{Alignment, Margin, Rect}, prelude::{Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, symbols::line::HORIZONTAL, text::{Line, Span, Text}, widgets::{Block, Borders, Clear, Padding, Paragraph, Tabs}
};

use crate::{
    app::{App, AppTabs}, components::{self, static_widgets}, views::{documents, instances, tasks}, Frame
};

pub fn render(app: &mut App, f: &mut Frame) {
    let size = f.size();
    // divide the viewport in 3 chunks
    // first one renders the tabs
    // second one the tab contents
    // third one the status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3), // the first line is reserved for input, similar to nvim command input
        ])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let titles: Vec<Line> = ["Documents (d)", "Indices (x)", "Tasks (t)", "Instances (i)"]
        .iter()
        .map(|t| {
            Line::from(Span::styled(
                format!("{}", t),
                Style::default().fg(Color::DarkGray),
            ))
        })
        .collect();

    // add instance info to the right most side of the tab bar
    let index: usize = match app.selected_tab {
        AppTabs::DocumentsTab => 0,
        AppTabs::IndicesTab => 1,
        AppTabs::TasksTab => 2,
        AppTabs::InstancesTab => 3,

    };


    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                // .borders(Borders::BOTTOM)
                .title(" MeiliFinder ")
                .title_style(Style::default().fg(Color::Magenta))
                .bold(),
        )
        .select(index)
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green));


    // divide the top portion in tabs bar and instance info:
    let top_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([ratatui::layout::Constraint::Percentage(75), ratatui::layout::Constraint::Percentage(25)])
        .split(chunks[0]);


    f.render_widget(tabs, top_chunks[0]);
    

    let instance_widget = Paragraph::new(
        Line::from(
            vec![
                Span::styled("● ", Style::default().fg(Color::Green)),
                Span::from(format!("Instance: {}", app.current_instance.name.clone()))
            ]

        )
    ).block(Block::default()
                // .borders(Borders::BOTTOM)
            .padding(Padding::new(1, 0, 1, 1))) // due to bottom border, no padding is applied on that side
            .alignment(Alignment::Right)
                ;

    f.render_widget(instance_widget, top_chunks[1]);




    // draw content based on the selected tab
    match app.selected_tab {
        AppTabs::DocumentsTab => documents::draw_documents(f, chunks[1], app),
        AppTabs::IndicesTab => instances::draw_instances(f, app),
        AppTabs::TasksTab => tasks::draw_tasks(f, chunks[1], app),
        AppTabs::InstancesTab => instances::draw_instances(f, app),
    };


    //Status bar
    let app_mode_indicator: Vec<Span> = match app.app_mode {
        crate::app::AppMode::Normal => {
            vec![
            Span::styled(format!(" NORMAL "), Style::default().bg(Color::Blue).bold()),
            Span::styled(
            format!(" Q: close app. Tab: switch tabs. Enter: de/select document. R: reload results. S: search mode. "),
            Style::default()),
            Span::styled(
            format!("Last refreshed: {}", &app.last_refreshed),
            Style::default().fg(Color::LightCyan))]
        }
        crate::app::AppMode::Search => {
            vec![
                Span::styled(format!(" SEARCH "), Style::default().bg(Color::Red).bold()),
                Span::styled(
                    format!(
                        " Esc: back to normal mode. Tab: navigate forms. Enter: submit search."
                    ),
                    Style::default(),
                ),
            ]
        }
    };


    let status_bar = Line::from(app_mode_indicator);

    // first part goes to the input field
    let bottom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    // input bar
    if app.app_mode == crate::app::AppMode::Search {
        components::input_bar::draw_input_bar(f, bottom_chunks[0], app);
    }

    f.render_widget(Paragraph::new(status_bar), bottom_chunks[1]);

}


