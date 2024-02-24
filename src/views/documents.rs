use ratatui::{layout::Rect, style::{Color, Style}, widgets::{Block, Borders, Clear, Padding}};

use crate::{app::App, components::static_widgets, Frame};




pub fn draw_documents(f: &mut Frame, chunk: Rect, app: &App){


    // let popup_title = " Documents ";

    // let block = Block::default()
    //     .title(popup_title)
    //     .borders(Borders::ALL)
    //     .padding(Padding::new(2, 2, 1, 1))
    //     .style(Style::default().fg(Color::DarkGray));

    // f.render_widget(block, chunk);



    // first chuck is reserved for the search and other query details
    let document_chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([ratatui::layout::Constraint::Percentage(25), ratatui::layout::Constraint::Percentage(75)])
        .split(chunk);


    let search_block = Block::default()
        .title(" Search Parameters (s) ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(search_block, document_chunks[0]);


    // second chunk is reserved for the list of documents from search Results


    let list_block = Block::default()
        .title(" Documents ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(list_block, document_chunks[1]);



    




}