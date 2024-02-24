use ratatui::{
    layout::Rect, prelude::{Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, Borders, Clear, Padding, Paragraph, Tabs}
};

use crate::{
    app::{App, AppTabs}, components::static_widgets, views::{instances, tasks}, Frame
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
            Constraint::Length(1),
        ])
        .split(size);

    let block = Block::default();
    f.render_widget(block, size);

    let titles: Vec<Line> = ["Documents (d)", "Indices (x)", "Tasks (t)", "Instances (i)"]
        .iter()
        .map(|t| {
            Line::from(Span::styled(
                format!("{}", t),
                Style::default().fg(Color::LightCyan),
            ))
        })
        .collect();
    let index: usize = match app.selected_tab {
        AppTabs::DocumentsTab => 0,
        AppTabs::IndicesTab => 1,
        AppTabs::TasksTab => 2,
        AppTabs::InstancesTab => 3,

    };


    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Meili4u "),
        )
        .select(index)
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Green));

    f.render_widget(tabs, chunks[0]);


    // draw content based on the selected tab
    match app.selected_tab {
        AppTabs::DocumentsTab => instances::draw_instances(f, app),
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

    f.render_widget(Paragraph::new(status_bar), chunks[2]);

    //SEARCH MODAL
    //todo: move to its own component

    if app.app_mode == crate::app::AppMode::Search {
        let popup_title = " ⌕ Search Query ";

        let area = static_widgets::centered_rect(69, 50, f.size()); //size of the MODAL

        // divide the MODAL in 4 chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3)])
            .split(area);

        f.render_widget(Clear, area); //this clears out the background

        // search query section
        let mut text = Text::from(Line::from(app.query.clone()));
        text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

        let query_field_color = if app.current_search_form == crate::app::SearchForm::Query {
            Color::Yellow
        } else {
            Color::LightCyan
        };

        let input_field = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(popup_title))
            .style(Style::default().fg(query_field_color))
            .alignment(ratatui::prelude::Alignment::Left);

        f.render_widget(input_field, chunks[0]);

        //filter query section

        let mut filter_query_text = Text::from(Line::from(app.filter_query.clone()));
        filter_query_text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

        let filter_field_color = if app.current_search_form == crate::app::SearchForm::Filter {
            Color::Yellow
        } else {
            Color::LightCyan
        };

        let filter_query_input_field = Paragraph::new(filter_query_text)
            .block(Block::default().borders(Borders::ALL).title(" ¥ Filter Query "))
            .style(Style::default().fg(filter_field_color))
            .alignment(ratatui::prelude::Alignment::Left);

        f.render_widget(filter_query_input_field, chunks[1]);

        //sort query section

        let mut sort_query_text = Text::from(Line::from(app.sort_query.clone()));
        sort_query_text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));


        let sort_field_color = if app.current_search_form == crate::app::SearchForm::Sort {
            Color::Yellow
        } else {
            Color::LightCyan
        };

        let sort_query_input_field = Paragraph::new(sort_query_text)
            .block(Block::default().borders(Borders::ALL).title(" ↑↓ Sort Query "))
            .style(Style::default().fg(sort_field_color))
            .alignment(ratatui::prelude::Alignment::Left);

        f.render_widget(sort_query_input_field, chunks[2]);

        // cursor should be drawn only when the current form is selected

        set_cursor_position(f, app, &chunks);

    }
}



       fn set_cursor_position(f: &mut Frame, app: &App, chunks: &[Rect]) {
            match app.current_search_form {
                crate::app::SearchForm::Query => {
                    f.set_cursor(
                        // Draw the cursor at the current position in the input field.
                        // This position is can be controlled via the left and right arrow key
                        chunks[0].x + app.cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[0].y + 1,
                    );
                }
                crate::app::SearchForm::Filter => {
                    f.set_cursor(
                        // Draw the cursor at the current position in the input field.
                        // This position is can be controlled via the left and right arrow key
                        chunks[1].x + app.filter_cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[1].y + 1,
                    );
                }
                crate::app::SearchForm::Sort => {
                    f.set_cursor(
                        // Draw the cursor at the current position in the input field.
                        // This position is can be controlled via the left and right arrow key
                        chunks[2].x + app.sort_cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[2].y + 1,
                    );
                }
            }
        }