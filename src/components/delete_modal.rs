use ratatui::{prelude::*, widgets::*};

use crate::{app::App, constants::DELETE_MODE_COLOR, Frame};


pub fn render_delete_modal(f: &mut Frame, area: Rect, app: &App) {

    let delete_modal_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(area);

    let document_info = app.get_current_document_info();

    let document_info = Paragraph::new(format!("{}", document_info))
        .block(Block::default()
            .title(" Delete Preview ")
            .padding(Padding::uniform(1))
            .fg(DELETE_MODE_COLOR))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(document_info, delete_modal_area[0]);


    let action_button_info = Paragraph::new(Line::from(
        Span::styled(
            "<del> or <backspace> to confirm",
            Style::default().fg(DELETE_MODE_COLOR)
        )

    )).alignment(Alignment::Center);

    f.render_widget(action_button_info, delete_modal_area[1]);


}