use std::iter;
use tui_scrollview::{ScrollView, ScrollViewState};
use ratatui::{prelude::*, widgets::*};

use crate::Frame;




fn render_document_preview(frame: &mut Frame, area: Rect) {

    // let size = Size::new(10, 100);
    // let mut scroll_view = ScrollView::new(size);

    // let some_long_string =
    //     iter::repeat("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n")
    //        .take(100)
    //        .collect::<String>();
    // // let area = Rect::new(0, 0, 10, 100);
    // scroll_view.render_widget(Paragraph::new(some_long_string), area);
    // let mut state = ScrollViewState::default();
    // frame.render_stateful_widget(scroll_view, area, &mut state);
}