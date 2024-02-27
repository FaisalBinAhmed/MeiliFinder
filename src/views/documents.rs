use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{block::{Position, Title}, Block, Borders, Clear, Padding, Paragraph}};

use crate::{app::{App, SearchForm}, components::static_widgets, Frame};




pub fn draw_documents(f: &mut Frame, chunk: Rect, app: &App){

    // first chuck is reserved for the search and other query details
    let document_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunk);


    let search_block = Block::default()
        // .title(" Search Parameters (s) ")
        .title_style(Style::default()
        // .fg(Color::Magenta)
        // .fg(Color::Black)
    )
        .title(Title::from(format!(" Search (s) in index: {} ({}) ", app.current_index, 23000)).position(Position::Top).alignment(Alignment::Center))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray));

    // lets render the block with bordersq
    f.render_widget(search_block, document_chunks[0]);

    // then we override the middle part
    
    let search_block_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .margin(2)
            .split(document_chunks[0]);


    draw_search_parameters(f, search_block_chunks[0], app);

        // right chunk is reserved for the current index & its details
    //     let current_index_block = Block::default()
    //     // .title(format!("Index: {}", app.current_index))
    //     // .borders(Borders::ALL)
    //     .style(Style::default().bg(Color::Yellow));

    // f.render_widget(current_index_block, search_block_chunks[1]);

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
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(list_block, document_chunks[1]);


}

fn get_search_form_color(current_search_form: &crate::app::SearchForm, form_type: SearchForm) -> Color {
    if current_search_form == &form_type {
        Color::Green
    } else {
        Color::White
    }
}

fn draw_search_parameters(f: &mut Frame, chunk: Rect, app: &App){

    // split this area
    let query_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(25),Constraint::Percentage(25), Constraint::Percentage(25) ])
        .split(chunk);


    let query_field_color = get_search_form_color(&app.current_search_form, SearchForm::Query);
    let filter_field_color = get_search_form_color(&app.current_search_form, SearchForm::Filter);
    let sort_field_color = get_search_form_color(&app.current_search_form, SearchForm::Sort);


    let input_field = create_input_field("⌕ Search Query: ", &app.query, query_field_color);
    f.render_widget(input_field, query_chunks[0]);

    let filter_query_input_field = create_input_field("¥ Filter Query: ", &app.filter_query, filter_field_color);
    f.render_widget(filter_query_input_field, query_chunks[1]);

    let sort_query_input_field = create_input_field("↑↓ Sort Query: ", &app.sort_query, sort_field_color);
    f.render_widget(sort_query_input_field, query_chunks[2]);

}

fn create_input_field<'a>(title: &'a str, value: &'a str, color: Color) -> Paragraph<'a> {
    Paragraph::new(Line::from(vec![
            Span::from(format!("{} {}", title, value))
        ]))
            .style(Style::default().fg(color))
            .alignment(ratatui::prelude::Alignment::Left)
}