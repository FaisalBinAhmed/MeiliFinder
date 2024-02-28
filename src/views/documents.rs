use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{block::{Position, Title}, Block, Borders, Clear, List, ListItem, Padding, Paragraph}};

use crate::{app::{App, SearchForm}, components::static_widgets, Frame};




pub fn draw_documents(f: &mut Frame, chunk: Rect, app: &App){

    // first chuck is reserved for the search and other query details
    let document_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(chunk);


    let search_block = Block::default()
        // .title(" Search Parameters (s) ")
        .title_style(Style::default()
        // .fg(Color::Magenta)
        // .fg(Color::Black)
    )
        .title(Title::from(format!(" Search (s) in index: {} ({}) ", app.current_index, 23000)).position(Position::Top).alignment(Alignment::Center))
        .borders(Borders::TOP)
        .style(Style::default().fg(Color::DarkGray));

    // lets render the block with bordersq
    f.render_widget(search_block, document_chunks[0]);

    // then we override the middle part
    
    let search_block_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .margin(1)
            .split(document_chunks[0]);


    draw_search_parameters(f, search_block_chunks[0], app);

    let index_info_paragraph = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(format!(" Index: {} ({}) ", app.current_index, 23000), Style::default().fg(Color::Blue)
        // .bg(Color::Yellow)
    ),
        // Span::styled(format!("Documents: {}", 23000), Style::default().fg(Color::Black).bg(Color::Yellow)),
    ]).alignment(Alignment::Right)
));

    f.render_widget(index_info_paragraph, search_block_chunks[1]);

    // second chunk is reserved for the list of documents from search Results
    let list_block = Block::default()
        .title(Title::from(" Documents ").position(Position::Top).alignment(Alignment::Center))
        .borders(Borders::TOP)
        .style(Style::default().fg(Color::DarkGray));

    let document_list = List::new(app.documents.iter().map(|d| {
        let pretty_json = match serde_json::to_string_pretty(&d) {
            Ok(json) => json,
            Err(_) => d.to_string(),
        };

        ListItem::new(
            Text::styled(pretty_json, Style::default())
        )
    })
    .collect::<Vec<ListItem>>())
    .block(list_block)
    .highlight_style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED))
    .style(Style::default().fg(Color::White));

    let list_state = &mut app.documents_scroll_state.clone();

    f.render_stateful_widget(document_list, document_chunks[1], list_state)


}

fn get_search_form_color(current_search_form: &crate::app::SearchForm, form_type: SearchForm) -> Color {
    if current_search_form == &form_type {
        Color::Green
    } else {
        Color::White
    }
}

fn draw_search_parameters(f: &mut Frame, chunk: Rect, app: &App){

    let query_field_color = get_search_form_color(&app.current_search_form, SearchForm::Query);
    let filter_field_color = get_search_form_color(&app.current_search_form, SearchForm::Filter);
    let sort_field_color = get_search_form_color(&app.current_search_form, SearchForm::Sort);


    let input_field = create_input_field("⌕ Search Query: ", &app.query, query_field_color);
    let filter_query_input_field = create_input_field("¥ Filter Query: ", &app.filter_query, filter_field_color);
    let sort_query_input_field = create_input_field("↑↓ Sort Query: ", &app.sort_query, sort_field_color);

    let parameter_paragraph = Paragraph::new(Text::from(vec![
        input_field,
        filter_query_input_field,
        sort_query_input_field
    ]));

    f.render_widget(parameter_paragraph, chunk);

}

fn create_input_field<'a>(title: &'a str, value: &'a str, color: Color) -> Line<'a> {
    Line::from(vec![
            Span::styled(title, Style::default().fg(color)),
            Span::styled(value, Style::default().fg(Color::DarkGray))
        ])
}