use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Color;

use crate::event::Event;

use super::app::{App, Toast};

impl App {
    pub fn remove_toast_with_delay(&mut self) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let _ = sender.send(Event::Key(KeyEvent::from(KeyCode::ScrollLock)));
            // this keypress is just to trigger the event handler to remove the toast
        });
    }

    pub fn remove_toast(&mut self) {
        self.toast = None
    }

    pub fn show_toast(&mut self, message: String, color: Color) {
        self.toast = Some(Toast { message, color });
        self.remove_toast_with_delay();
    }
}
