// use std::{borrow::BorrowMut, iter};
// use tui_scrollview::{ScrollView, ScrollViewState};
use ratatui::{prelude::*, widgets::*};
// use tui_textarea::TextArea;

use crate::{app::app::App, constants::PREVIEW_MODE_COLOR, Frame};


pub fn render_document_preview(f: &mut Frame, area: Rect, app: &App) {

    // let size = Size::new(area.width, area.height);
    // let size = Size::new(300, 300);
    // let mut scroll_view = ScrollView::new(size);

    // let some_long_string =
    //     iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
    //        .take(10)
    //        .collect::<String>();
    // let area = Rect::new(0, 0, 200, 200);
    // let block = Block::default()
    //     .title("Document Preview").borders(Borders::ALL);

    // scroll_view.render_widget(Paragraph::new(some_long_string).block(block), area);
    // let mut state = ScrollViewState::default();
    // f.render_stateful_widget(scroll_view, area, &mut state);

    // let list_state = &mut app.action_scroll_view_state.clone();

    // f.render_stateful_widget(scroll_view, area, list_state);



    // let mut textarea = TextArea::default();
    // action_text_area.set_block(
    //     Block::default()
    //         .borders(Borders::ALL)
    //         .title(" Action "),
    // );

    // let text = "This is a minimal example of a textarea.\nYou can use the arrow keys to move the cursor and type text.\nPress `Ctrl + A` to move to the beginning of the line and `Ctrl + E` to move to the end.\nPress `Ctrl + U` to delete the text from the cursor to the beginning of the line.\nPress `Ctrl + K` to delete the text from the cursor to the end of the line.\nPress `Ctrl + W` to delete the previous word.\nPress `Ctrl + L` to clear the screen.\nPress `Ctrl + C` to exit the application.";
    
    // let text = app.get_current_document_info();

    // action_text_area.insert_str(text);

    // f.render_widget(action_text_area.widget(), area);


    let document_preview_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(area);

    let document_info = app.get_current_document_info();

    let document_info = Paragraph::new(format!("{}", document_info))
        .block(Block::default()
            .title(" Document Preview ")
            .padding(Padding::uniform(1))
            .fg(PREVIEW_MODE_COLOR))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(document_info, document_preview_area[0]);


    let action_button_info = Paragraph::new(Line::from(
        Span::styled(
            "Press <e> to edit (coming soon) | <del> or <backspace> to delete | <esc> to close preview",
            Style::default().fg(PREVIEW_MODE_COLOR)
        )

    )).alignment(Alignment::Center);

    f.render_widget(action_button_info, document_preview_area[1]);


}