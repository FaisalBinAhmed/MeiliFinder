use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Borders, Clear, Padding, Paragraph, Tabs},
};

use crate::{
    app::{App, AppMode, AppTabs},
    components::{
        document_preview::render_document_preview, static_widgets::centered_rect, status_bar,
    },
    constants::{ACTION_MODE_COLOR, INSTANCE_COLOR},
    views::{documents, indices, instances, tasks},
    Frame,
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
                    Span::styled(" MeiliFinder", Style::default().fg(Color::LightMagenta)),
                    Span::styled(" ʕʘ̅͜ʘ̅ʔ ", Style::default().fg(Color::White)),
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
        .constraints([
            ratatui::layout::Constraint::Percentage(70),
            ratatui::layout::Constraint::Percentage(30),
        ])
        .split(chunks[0]);

    f.render_widget(tabs, top_chunks[0]);

    let instance_widget = Paragraph::new(Line::from(vec![
        Span::styled("● ", Style::default().fg(Color::Green)),
        Span::raw("MeiliSearch instance: "),
        Span::styled(
            format!("{} ", app.current_instance.name.clone()),
            Style::default().fg(INSTANCE_COLOR).bold(),
        ),
    ]))
    .block(Block::default().padding(Padding::new(1, 0, 1, 1))) // due to bottom border, no padding is applied on that side
    .alignment(Alignment::Right);

    f.render_widget(instance_widget, top_chunks[1]);

    // draw content based on the selected tab
    match app.selected_tab {
        AppTabs::DocumentsTab => documents::draw_documents(f, chunks[1], app),
        AppTabs::IndicesTab => indices::draw_indices(f, chunks[1], app),
        AppTabs::TasksTab => tasks::draw_tasks(f, chunks[1], app),
        AppTabs::InstancesTab => instances::draw_instances(f, chunks[1], app),
    };

    //Status bar
    status_bar::draw_status_bar(f, chunks[2], app);

    // action mode ui overwrites the full app ui, like a modal
    if app.app_mode == AppMode::Action {
        let action_modal_area = centered_rect(69, 69, f.size()); //size of the MODAL

        let action_modal = Block::default()
            .title(" Action ")
            .borders(Borders::ALL)
            // .border_type(ratatui::widgets::BorderType::Rounded)
            // .border_style(Style::default().fg(Color::Rgb(255, 205, 170)))
            .style(Style::default().fg(ACTION_MODE_COLOR))
            .padding(Padding::uniform(1));

        f.render_widget(Clear, action_modal_area); //this clears out the background
        f.render_widget(action_modal, action_modal_area);

        // app.action_text_area.insert_str(app.get_current_document_info());

        render_document_preview(f, action_modal_area, app);
    }
}
