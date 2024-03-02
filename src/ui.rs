use ratatui::{prelude::*, widgets::{block::Title, Block, Padding, Paragraph, Tabs}};

use crate::{
    app::{App, AppTabs}, components::status_bar, views::{documents, indices, instances, tasks}, Frame
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
            Constraint::Length(4), // the first line is reserved for input, similar to nvim command input
        ])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let titles: Vec<Line> = ["Documents <d>", "Indices <x>", "Tasks <t>", "Instances <i>"]
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
                // .title(" MeiliFinder ")
                .title(Title::from(vec![
                    Span::styled(" Meili", Style::default().fg(Color::LightMagenta)),
                    Span::styled("Finder ", Style::default().fg(Color::White)),
                    // Span::styled("A Meilisearch client", Style::default().fg(Color::White)),
                ]))
                // .title_style(Style::default().fg(Color::LightMagenta))
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
                Span::raw("Instance: "),
                Span::styled(format!("{}", app.current_instance.name.clone()), Style::default().bold())
            ]

        )
    ).block(Block::default()
            .padding(Padding::new(1, 0, 1, 1))) // due to bottom border, no padding is applied on that side
            .alignment(Alignment::Right);

    f.render_widget(instance_widget, top_chunks[1]);




    // draw content based on the selected tab
    match app.selected_tab {
        AppTabs::DocumentsTab => documents::draw_documents(f, chunks[1], app),
        AppTabs::IndicesTab => indices::draw_indices(f, chunks[1], app),
        AppTabs::TasksTab => tasks::draw_tasks(f, chunks[1], app),
        AppTabs::InstancesTab => instances::draw_instances(f, app), // todo
    };


    //Status bar
    status_bar::draw_status_bar(f, chunks[2], app);

}


