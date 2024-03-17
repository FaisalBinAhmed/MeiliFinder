
use super::app::{App, SearchForm};

impl App {
    pub fn enter_char(&mut self, new_char: char) {
        if new_char.len_utf8() == 1 {
            // temporary workaround: ignoring non-ascii characters that are more than 1 byte

            match self.current_search_form {
                SearchForm::Query => {
                    self.query.insert(self.cursor_position, new_char);
                    self.move_cursor_right();
                }
                SearchForm::Filter => {
                    self.filter_query
                        .insert(self.filter_cursor_position, new_char);
                    self.move_cursor_right();
                }
                SearchForm::Sort => {
                    self.sort_query.insert(self.sort_cursor_position, new_char);
                    self.move_cursor_right();
                }
            }
        }
        //should also commence the search
    }

    fn delete_char(query: &mut String, cursor_position: usize) {
        let is_not_cursor_leftmost = cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = query.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = query.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            let new_query: std::string::String =
                before_char_to_delete.chain(after_char_to_delete).collect();
            query.clear();
            query.push_str(&new_query);
        }
    }

    pub fn delete_char_for_form(&mut self) {
        match self.current_search_form {
            SearchForm::Query => {
                Self::delete_char(&mut self.query, self.cursor_position);
                self.move_cursor_left();
            }
            SearchForm::Filter => {
                Self::delete_char(&mut self.filter_query, self.filter_cursor_position);
                self.move_cursor_left();
            }
            SearchForm::Sort => {
                Self::delete_char(&mut self.sort_query, self.sort_cursor_position);
                self.move_cursor_left();
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self.current_search_form {
            SearchForm::Query => {
                let cursor_moved_left = self.cursor_position.saturating_sub(1);
                self.cursor_position = self.clamp_cursor(cursor_moved_left);
            }
            SearchForm::Filter => {
                let cursor_moved_left = self.filter_cursor_position.saturating_sub(1);
                self.filter_cursor_position = self.clamp_cursor(cursor_moved_left);
            }
            SearchForm::Sort => {
                let cursor_moved_left = self.sort_cursor_position.saturating_sub(1);
                self.sort_cursor_position = self.clamp_cursor(cursor_moved_left);
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.current_search_form {
            SearchForm::Query => {
                let cursor_moved_right = self.cursor_position.saturating_add(1);
                self.cursor_position = self.clamp_cursor(cursor_moved_right);
            }
            SearchForm::Filter => {
                let cursor_moved_right = self.filter_cursor_position.saturating_add(1);
                self.filter_cursor_position = self.clamp_cursor(cursor_moved_right);
            }
            SearchForm::Sort => {
                let cursor_moved_right = self.sort_cursor_position.saturating_add(1);
                self.sort_cursor_position = self.clamp_cursor(cursor_moved_right);
            }
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        match self.current_search_form {
            SearchForm::Query => new_cursor_pos.clamp(0, self.query.chars().count()),
            SearchForm::Filter => new_cursor_pos.clamp(0, self.filter_query.chars().count()),
            SearchForm::Sort => new_cursor_pos.clamp(0, self.sort_query.chars().count()),
        }
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
}
