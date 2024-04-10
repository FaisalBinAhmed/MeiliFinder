use ratatui::{prelude::*, widgets::*};

use crate::{app::app::App, constants::PREVIEW_MODE_COLOR, Frame};

pub fn render_document_preview(f: &mut Frame, area: Rect, app: &App) {
    let document_preview_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(area);

    let document_info = app.get_current_document_info();
    let document_lines = document_info
                                    .lines()
                                    .collect::<Vec<&str>>();

    let list_widget = List::new(document_lines.iter().map(|line| {
        ListItem::new(Text::raw(*line))
    })).highlight_style(
        ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::REVERSED),
    );




    // let document_info = Paragraph::new(format!("{}", document_info))
    //     .block(
    //         Block::default()
    //             .title(" Document Preview ")
    //             .padding(Padding::uniform(1))
    //             .fg(PREVIEW_MODE_COLOR),
    //     )
    //     .style(Style::default().fg(Color::White))
    //     .wrap(Wrap { trim: false });

    // f.render_widget(document_info, document_preview_area[0]);

    // let action_button_info = Paragraph::new(Line::from(Span::styled(
    //     "Press <e> to edit (coming soon) | <backspace> to delete",
    //     Style::default().fg(PREVIEW_MODE_COLOR),
    // )))
    // .alignment(Alignment::Center);

    // f.render_widget(action_button_info, document_preview_area[1]);

   let list_state = &mut app.preview_scroll_state.clone();

    f.render_stateful_widget(list_widget, area, list_state);

}
