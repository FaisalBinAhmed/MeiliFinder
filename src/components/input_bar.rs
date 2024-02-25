use ratatui::{layout::Rect, style::{Modifier, Style}, text::{Line, Text}, widgets::Paragraph};

use crate::{app::{self, App}, Frame};





pub fn draw_input_bar(f: &mut Frame, chunk: Rect, app: &App) {
    let mut input_field: Text<'_> = match app.current_search_form {
        app::SearchForm::Query => {
            let query_text = Text::from(format!("⌕ Search Query: {} ", app.query.clone()));
            query_text
        }
        app::SearchForm::Filter => {
            let filter_query_text = Text::from(format!("¥ Filter Query: {} ", app.filter_query.clone()));
            filter_query_text
        }
        app::SearchForm::Sort => {
            let sort_query_text = Text::from(format!("↑↓ Sort Query: {} ", app.sort_query.clone()));
            sort_query_text
        }
    };

    input_field.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

    let input_paragraph = Paragraph::new(input_field)
        .alignment(ratatui::prelude::Alignment::Left);

    f.render_widget(input_paragraph, chunk);
    set_cursor_position(f, app, chunk);
}



       fn set_cursor_position(f: &mut Frame, app: &App, chunk: Rect) {
            match app.current_search_form {
                crate::app::SearchForm::Query => {
                    f.set_cursor(
                        // Draw the cursor at the current position in the input field.
                        // This position is can be controlled via the left and right arrow key
                        chunk.x + app.cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunk.y + 1,
                    );
                }
                crate::app::SearchForm::Filter => {
                    f.set_cursor(
                        // Draw the cursor at the current position in the input field.
                        // This position is can be controlled via the left and right arrow key
                        chunk.x + app.filter_cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunk.y + 1,
                    );
                }
                crate::app::SearchForm::Sort => {
                    f.set_cursor(
                        // Draw the cursor at the current position in the input field.
                        // This position is can be controlled via the left and right arrow key
                        chunk.x + app.sort_cursor_position as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunk.y + 1,
                    );
                }
            }
        }




    // if app.app_mode == crate::app::AppMode::Search {
        
    //     let popup_title = " ⌕ Search Query ";

    //     let area = static_widgets::centered_rect(69, 50, f.size()); //size of the MODAL

    //     // divide the MODAL in 4 chunks
    //     let chunks = Layout::default()
    //         .direction(Direction::Vertical)
    //         .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3)])
    //         .split(area);

    //     f.render_widget(Clear, area); //this clears out the background

    //     // search query section
    //     let mut text = Text::from(Line::from(app.query.clone()));
    //     text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

    //     let query_field_color = if app.current_search_form == crate::app::SearchForm::Query {
    //         Color::Yellow
    //     } else {
    //         Color::LightCyan
    //     };

    //     let input_field = Paragraph::new(text)
    //         .block(Block::default().borders(Borders::ALL).title(popup_title))
    //         .style(Style::default().fg(query_field_color).bg(Color::Blue))
    //         .alignment(ratatui::prelude::Alignment::Left);

    //     f.render_widget(input_field, chunks[0]);

    //     //filter query section

    //     let mut filter_query_text = Text::from(Line::from(app.filter_query.clone()));
    //     filter_query_text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

    //     let filter_field_color = if app.current_search_form == crate::app::SearchForm::Filter {
    //         Color::Yellow
    //     } else {
    //         Color::LightCyan
    //     };

    //     let filter_query_input_field = Paragraph::new(filter_query_text)
    //         .block(Block::default().borders(Borders::ALL).title(" ¥ Filter Query "))
    //         .style(Style::default().fg(filter_field_color))
    //         .alignment(ratatui::prelude::Alignment::Left);

    //     f.render_widget(filter_query_input_field, chunks[1]);

    //     //sort query section

    //     let mut sort_query_text = Text::from(Line::from(app.sort_query.clone()));
    //     sort_query_text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));


    //     let sort_field_color = if app.current_search_form == crate::app::SearchForm::Sort {
    //         Color::Yellow
    //     } else {
    //         Color::LightCyan
    //     };

    //     let sort_query_input_field = Paragraph::new(sort_query_text)
    //         .block(Block::default().borders(Borders::ALL).title(" ↑↓ Sort Query "))
    //         .style(Style::default().fg(sort_field_color))
    //         .alignment(ratatui::prelude::Alignment::Left);

    //     f.render_widget(sort_query_input_field, chunks[2]);

    //     // cursor should be drawn only when the current form is selected

    //     set_cursor_position(f, app, &chunks);

    // }

