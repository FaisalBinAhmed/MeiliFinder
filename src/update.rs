use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
// use tui_textarea::Scrolling;

use crate::{
    app::{App, AppMode},
    event::Event,
};

//todo: should_redraw refactor
pub async fn update(app: &mut App, key_event: KeyEvent) {
    match app.app_mode {
        AppMode::Normal => match key_event.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('s') => {
                app.app_mode = AppMode::Search;
                app.should_redraw = true;
            }
            KeyCode::Char(' ') => {
                // todo: actions
                app.app_mode = AppMode::Action;
                app.should_redraw = true;
            }
            KeyCode::Char('r') => {
                // app.update_departures().await;
                // app.should_redraw = true;
            }
            KeyCode::Down => {
                //todo: this should be a match statement depending on the current tab
                app.increment_scroll_state();
                app.should_redraw = true;
            }
            KeyCode::Up => {
                app.decrement_scroll_state();
                app.should_redraw = true;
            }
            KeyCode::Enter => {
                // this is used to change current index or instances depending on the current tab
                app.select_item();
                app.should_redraw = true;
            }
            KeyCode::Tab => {
                app.toggle_tabs();
                app.should_redraw = true;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            // tab shortcuts
            KeyCode::Char('d') => {
                app.selected_tab = crate::app::AppTabs::DocumentsTab;
                app.should_redraw = true;
            }
            KeyCode::Char('x') => {
                app.selected_tab = crate::app::AppTabs::IndicesTab;
                app.should_redraw = true;
            }
            KeyCode::Char('t') => {
                app.selected_tab = crate::app::AppTabs::TasksTab;
                app.should_redraw = true;
            }
            KeyCode::Char('i') => {
                app.selected_tab = crate::app::AppTabs::InstancesTab;
                app.should_redraw = true;
            }
            _ => {
                // todo: pass the key event?
            }
        },
        AppMode::Search => match key_event.code {
            KeyCode::Enter => {
                // commence search
                app.search_documents().await;
                app.app_mode = AppMode::Normal;
                app.should_redraw = true;
            }
            KeyCode::Char(to_insert) => {
                // app.search_scroll_state = ListState::default();
                app.enter_char(to_insert);
                app.should_redraw = true;
            }
            KeyCode::Backspace => {
                // app.search_scroll_state = ListState::default();
                app.delete_char();
                app.should_redraw = true;
            }
            KeyCode::Tab => {
                app.switch_search_form();
                app.should_redraw = true;
            }
            KeyCode::Down => {
                // app.scroll_down();
                app.should_redraw = true;
            }
            KeyCode::Up => {
                // app.scroll_up();
                app.should_redraw = true;
            }
            KeyCode::Left => {
                // app.move_cursor_left();
                app.should_redraw = true;
            }
            KeyCode::Right => {
                // app.move_cursor_right();
                app.should_redraw = true;
            }
            KeyCode::Esc => {
                app.app_mode = AppMode::Normal;
                app.should_redraw = true;
            }
            _ => {}
        },
        AppMode::Action => match key_event.code {
            KeyCode::Enter => {
                // commence action
                // app.perform_action().await;
                // app.app_mode = AppMode::Normal;
                // app.action_text_area.insert_newline();
                app.should_redraw = true;
            }
            KeyCode::Char(to_insert) => {
                // app.search_scroll_state = ListState::default();
                // app.enter_char(to_insert);
                // app.action_text_area.insert_char(to_insert);
                app.should_redraw = true;
            }
            KeyCode::Backspace => {
                // app.search_scroll_state = ListState::default();
                // app.delete_item();
                // app.action_text_area.delete_char();
                app.should_redraw = true;
            }
            KeyCode::Esc => {
                app.app_mode = AppMode::Normal;
                app.should_redraw = true;
            }
            KeyCode::Up => {
                // app.action_text_area.scroll(Scrolling::Delta { rows: -1, cols: 0 });
                app.should_redraw = true;
            }
            KeyCode::Down => {
                // app.action_text_area.scroll(Scrolling::Delta { rows: 1, cols: 0 });
                app.should_redraw = true;
            }
            _ => {}
        },
    }
}

// this lets us mutate the app state without having to pass a mutable reference and blocking the main ui/event thread or having to use a mutex
// we simulate the refresh command by sending a key event to the event handler
// the event handler has a mutable reference to the app and can mutate it
pub fn initiate_auto_refresh(sender: tokio::sync::mpsc::UnboundedSender<Event>) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            // println!("sending refresh event");
            let _ = sender.send(Event::Key(KeyEvent::from(KeyCode::Char('r'))));
        }
    });
}
