use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, List, ListItem, Padding, Paragraph, Wrap,
    },
};

use crate::{
    app::{App, SearchForm},
    constants::INDEX_COLOR,
    Frame,
};

fn draw_index_bar(f: &mut Frame, chunk: Rect, app: &App) {
    let current_index: String = match &app.current_index {
        Some(index) => index.uid.clone(),
        None => "No index selected".to_string(),
    };

    let index_info = Line::from(vec![
        Span::styled(format!(" Search <s> in index: "), Style::default()),
        Span::styled(
            format!("{} ", current_index),
            Style::default().fg(INDEX_COLOR).bold(),
        ),
    ]);

    f.render_widget(Paragraph::new(index_info).alignment(Alignment::Left), chunk);
}

pub fn draw_documents(f: &mut Frame, chunk: Rect, app: &App) {
    // first chuck is reserved for the search and other query details
    let document_view_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(chunk);

    draw_index_bar(f, document_view_chunks[0], app);

    // then we override the middle part

    let search_block_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .margin(1)
        .split(document_view_chunks[1]);

    draw_search_parameters(f, search_block_chunks[0], app);

    let search_result_info_paragraph = Paragraph::new(vec![
        Line::from(Span::styled(
            format!(" Hits: {} of estimated {}", app.current_result_metadata.hits, app.current_result_metadata.estimated_total_hits),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            format!(" Time: {}ms", app.current_result_metadata.processing_time_ms),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(vec![
            Span::raw("<ctrl + p>"),
            Span::styled(" Bulk Delete", Style::default().fg(Color::Red)),
            Span::raw(" for this filter")
        ]),
    ])
    .alignment(Alignment::Right);

    f.render_widget(search_result_info_paragraph, search_block_chunks[1]);

    // second chunk is reserved for the list of documents from search Results

    // divide it hoizontally for list and preview

    let document_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        // .margin(1)
        .split(document_view_chunks[2]);

    let list_block = Block::default()
        .title(
            Title::from(" Documents ")
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        // .title_style(Style::default().fg(Color::Black).bg(Color::DarkGray))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        // .padding(Padding::uniform(1))
        .style(Style::default().fg(Color::DarkGray));

    let document_list = List::new(
        app.documents
            .iter()
            .map(|d| {
                let pretty_json = match serde_json::to_string_pretty(d) {
                    Ok(json) => json,
                    Err(_) => format!("{:#?}", d),
                };

                ListItem::new(Text::styled(pretty_json, Style::default()))
            })
            .collect::<Vec<ListItem>>(),
    )
    .block(list_block)
    // .highlight_style(ratatui::style::Style::default().bg(Color::Rgb(24, 24, 24)).fg(Color::White))
    // .style(Style::default().fg(Color::White));
    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
    .highlight_style(
        ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED),
    );

    let list_state = &mut app.documents_scroll_state.clone();

    f.render_stateful_widget(document_list, document_area[0], list_state);

    let document_info =
        ratatui::widgets::Paragraph::new(format!("{}", app.get_current_document_info()))
            .block(
                ratatui::widgets::Block::default()
                    .title(" Document Preview ")
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::uniform(1))
                    .fg(Color::DarkGray),
            )
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
            .wrap(Wrap { trim: false });

    f.render_widget(document_info, document_area[1]);
}

fn get_search_form_color(
    current_search_form: &crate::app::SearchForm,
    form_type: SearchForm,
) -> Color {
    if current_search_form == &form_type {
        Color::LightGreen
    } else {
        Color::DarkGray
    }
}

fn draw_search_parameters(f: &mut Frame, chunk: Rect, app: &App) {
    let query_field_color = get_search_form_color(&app.current_search_form, SearchForm::Query);
    let filter_field_color = get_search_form_color(&app.current_search_form, SearchForm::Filter);
    let sort_field_color = get_search_form_color(&app.current_search_form, SearchForm::Sort);

    let input_field = create_input_field("⌕  Search Query: ", &app.query, query_field_color);
    let filter_query_input_field =
        create_input_field("¥  Filter Query: ", &app.filter_query, filter_field_color);
    let sort_query_input_field =
        create_input_field("↑↓ Sort Query: ", &app.sort_query, sort_field_color);

    let parameter_paragraph = Paragraph::new(Text::from(vec![
        input_field,
        filter_query_input_field,
        sort_query_input_field
    ]))
    // .bg(Color::Rgb(54, 54, 54))
    ;

    f.render_widget(parameter_paragraph, chunk);
}

fn create_input_field<'a>(title: &'a str, value: &'a str, color: Color) -> Line<'a> {
    Line::from(vec![
        Span::styled(title, Style::default().fg(color)),
        Span::styled(value, Style::default().fg(Color::White).underlined()),
    ])
}
