use ratatui::widgets::ListState;

use crate::api::{self, get_task_by_id};


#[derive(PartialEq)] // need this to do binary comparison
pub enum AppTabs {
    DocumentsTab,
    IndicesTab,
    TasksTab,
    InstancesTab,
}

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Search,
}

#[derive(Debug, PartialEq)]
pub enum SearchForm {
    Query,
    Filter,
    Sort,
}

pub struct Instance {
    pub id: String, // unique id
    pub name: String, // name of the instance, optional
    pub host: String, // host url of the instance
    pub primary_key: String, // primary api key to access the instance
}

pub struct App {
    pub selected_tab: AppTabs,
    pub should_quit: bool,

    pub should_redraw: bool,
    pub status: String,
    pub app_mode: AppMode,

    pub documents: Vec<serde_json::Value>,
    pub documents_scroll_state: ListState,

    // search MODAL
    pub query: String,
    pub filter_query: String,
    pub sort_query: String,
    // cursor position for each input
    pub cursor_position: usize,
    pub filter_cursor_position: usize,
    pub sort_cursor_position: usize,

    pub current_search_form: SearchForm,

    pub last_refreshed: String,



    //tasks related
    pub tasks: Vec<meilisearch_sdk::tasks::Task>,
    pub task_scroll_state: ListState,
    pub current_task_info: Option<String>,
    // pub selected_task: usize,

    // pub instances: Vec<Instance>,
    pub current_instance: Instance,

    pub current_index: String,

}

impl App {
    pub async fn new() -> Self {
        Self {
            selected_tab: AppTabs::DocumentsTab, // check if there is an instance, if not, switch to instances tab
            should_quit: false,
            should_redraw: true,
            status: "Loading documents...".to_string(),
            app_mode: AppMode::Normal,

            documents: api::get_documents().await,
            documents_scroll_state: ListState::default(),

            last_refreshed: "".to_string(),

            // search MODAL
            query: "".to_string(),
            filter_query: "".to_string(),
            sort_query: "".to_string(),

            cursor_position: 0,
            filter_cursor_position: 0,
            sort_cursor_position: 0,

            current_search_form: SearchForm::Query,
        

            tasks: api::get_tasks().await,
            task_scroll_state: ListState::default(),
            current_task_info: get_task_by_id(1).await,

            //temp
            current_instance: Instance {
                id: "1".to_string(), 
                name: "Movies Production".to_string(),
                host: "localhost".to_string(),
                primary_key: "".to_string()
            },

            current_index: "movies".to_string(),
        }
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    pub fn toggle_tabs(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => self.selected_tab = AppTabs::IndicesTab,
            AppTabs::IndicesTab => self.selected_tab = AppTabs::TasksTab,
            AppTabs::TasksTab => self.selected_tab = AppTabs::InstancesTab,
            AppTabs::InstancesTab => self.selected_tab = AppTabs::DocumentsTab,
        }
    }

    pub fn switch_search_form(&mut self) {
        match self.current_search_form {
            SearchForm::Query => self.current_search_form = SearchForm::Filter,
            SearchForm::Filter => self.current_search_form = SearchForm::Sort,
            SearchForm::Sort => self.current_search_form = SearchForm::Query,
        }
    }

    pub fn increment_scroll_state(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => self.increment_document_scroll_state(),
            AppTabs::TasksTab => self.increment_task_scroll_state(),
            _ => {}
        }
    }

    pub fn decrement_scroll_state(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => self.decrement_document_scroll_state(),
            AppTabs::TasksTab => self.decrement_task_scroll_state(),
            _ => {}
        }
    }


     pub fn increment_task_scroll_state(&mut self) {
        let i = match self.task_scroll_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.task_scroll_state.select(Some(i));
    }

    pub fn increment_document_scroll_state(&mut self) {
        let i = match self.documents_scroll_state.selected() {
            Some(i) => {
                if i >= self.documents.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.documents_scroll_state.select(Some(i));
    }

    pub fn decrement_task_scroll_state(&mut self) {
        let i = match self.task_scroll_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.task_scroll_state.select(Some(i));
    }

    pub fn decrement_document_scroll_state(&mut self) {
        let i = match self.documents_scroll_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.documents.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.documents_scroll_state.select(Some(i));
    }
    
    pub fn enter_char(&mut self, new_char: char) {
        if new_char.len_utf8() == 1 {
            // temporary workaround: ignoring non-ascii characters that are more than 1 byte

            match self.current_search_form {
                SearchForm::Query => {
                    self.query.insert(self.cursor_position, new_char);
                    self.move_cursor_right();
                }
                SearchForm::Filter => {
                    self.filter_query.insert(self.filter_cursor_position, new_char);
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

    pub fn delete_char(&mut self) {

        match self.current_search_form {
            SearchForm::Query => {
                let is_not_cursor_leftmost = self.cursor_position != 0;
                if is_not_cursor_leftmost {
                    let current_index = self.cursor_position;
                    let from_left_to_current_index = current_index - 1;

                    // Getting all characters before the selected character.
                    let before_char_to_delete = self.query.chars().take(from_left_to_current_index);
                    // Getting all characters after selected character.
                    let after_char_to_delete = self.query.chars().skip(current_index);

                    // Put all characters together except the selected one.
                    // By leaving the selected one out, it is forgotten and therefore deleted.
                    self.query = before_char_to_delete.chain(after_char_to_delete).collect();
                    self.move_cursor_left();
                }
            }
            SearchForm::Filter => {
                let is_not_cursor_leftmost = self.filter_cursor_position != 0;
                if is_not_cursor_leftmost {
                    let current_index = self.filter_cursor_position;
                    let from_left_to_current_index = current_index - 1;

                    // Getting all characters before the selected character.
                    let before_char_to_delete = self.filter_query.chars().take(from_left_to_current_index);
                    // Getting all characters after selected character.
                    let after_char_to_delete = self.filter_query.chars().skip(current_index);

                    // Put all characters together except the selected one.
                    // By leaving the selected one out, it is forgotten and therefore deleted.
                    self.filter_query = before_char_to_delete.chain(after_char_to_delete).collect();
                    self.move_cursor_left();
                }
            }
            SearchForm::Sort => {
                let is_not_cursor_leftmost = self.sort_cursor_position != 0;
                if is_not_cursor_leftmost {
                    let current_index = self.sort_cursor_position;
                    let from_left_to_current_index = current_index - 1;

                    // Getting all characters before the selected character.
                    let before_char_to_delete = self.sort_query.chars().take(from_left_to_current_index);
                    // Getting all characters after selected character.
                    let after_char_to_delete = self.sort_query.chars().skip(current_index);

                    // Put all characters together except the selected one.
                    // By leaving the selected one out, it is forgotten and therefore deleted.
                    self.sort_query = before_char_to_delete.chain(after_char_to_delete).collect();
                    self.move_cursor_left();
                }
            }
        }

        // let is_not_cursor_leftmost = self.cursor_position != 0;
        // if is_not_cursor_leftmost {
        //     let current_index = self.cursor_position;
        //     let from_left_to_current_index = current_index - 1;

        //     // Getting all characters before the selected character.
        //     let before_char_to_delete = self.query.chars().take(from_left_to_current_index);
        //     // Getting all characters after selected character.
        //     let after_char_to_delete = self.query.chars().skip(current_index);

        //     // Put all characters together except the selected one.
        //     // By leaving the selected one out, it is forgotten and therefore deleted.
        //     self.query = before_char_to_delete.chain(after_char_to_delete).collect();
        //     self.move_cursor_left();
        // }

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
            SearchForm::Sort => new_cursor_pos.clamp(0, self.sort_query.chars().count())
        }
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
    
}
