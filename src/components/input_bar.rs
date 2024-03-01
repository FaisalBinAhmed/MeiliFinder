use ratatui::{prelude::*, widgets::Paragraph};

use crate::{app::{self, App}, Frame};


pub fn draw_input_bar(f: &mut Frame, chunk: Rect, app: &App) {
    let input_field: Text<'_> = match app.current_search_form {
        app::SearchForm::Query => Text::from(app.query.clone()),
        app::SearchForm::Filter => Text::from(app.filter_query.clone()),
        app::SearchForm::Sort => Text::from(app.sort_query.clone()),
    };

    // input_field.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

    let title = match app.current_search_form {
        app::SearchForm::Query => " ⌕ Search Query ",
        app::SearchForm::Filter => " ¥ Filter Query ",
        app::SearchForm::Sort => " ↑↓ Sort Query ",
    };

    let input_paragraph = Paragraph::new(input_field)
    .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title(title).border_style(Style::default().fg(Color::LightGreen)))
    // .style(Style::default().fg(ratatui::style::Color::Black).bg(ratatui::style::Color::White))
        .alignment(ratatui::prelude::Alignment::Left);

    f.render_widget(input_paragraph, chunk);
    set_cursor_position(f, app, chunk);
}



    // we can put the cursor in the right position by offsetting the input bar text size  TODO: 
    fn set_cursor_position(f: &mut Frame, app: &App, chunk: Rect) {
        let cursor_position = match app.current_search_form {
            crate::app::SearchForm::Query => app.cursor_position,
            crate::app::SearchForm::Filter => app.filter_cursor_position,
            crate::app::SearchForm::Sort => app.sort_cursor_position,
        };

        f.set_cursor(
            chunk.x + cursor_position as u16 + 1,
            chunk.y + 1,
        );
    }