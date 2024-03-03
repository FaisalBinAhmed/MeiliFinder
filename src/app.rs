use meilisearch_sdk::{Index, Settings};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

use crate::{api::{self, get_all_index_settings, get_client}, utilities::{config_handler::retrieve_instances_from_file, scrolling_handler::{scroll_state_decrementer, scroll_state_incrementer}}};



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

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    pub id: String, // unique id
    pub name: String, // name of the instance, optional
    pub host: String, // host url of the instance
    pub primary_key: String, // primary api key to access the instance
}

pub struct App {

    pub meili_client: meilisearch_sdk::client::Client,

    pub selected_tab: AppTabs,
    pub should_quit: bool,

    pub should_redraw: bool,
    // pub status: String,
    pub app_mode: AppMode,

    pub documents: Vec<serde_json::Value>,
    pub documents_scroll_state: ListState,

    // search parameters
    pub query: String,
    pub filter_query: String,
    pub sort_query: String,

    pub current_search_form: SearchForm,
    // cursor position for each input
    pub cursor_position: usize,
    pub filter_cursor_position: usize,
    pub sort_cursor_position: usize,

    pub last_refreshed: String,

    //tasks related
    pub tasks: Vec<meilisearch_sdk::tasks::Task>,
    pub task_scroll_state: ListState,

    // index related
    pub indices: Vec<Index>,
    pub indices_scroll_state: ListState,
    // not sure
    pub current_index: Option<Index>,

    pub all_index_settings: Vec<Settings>,


    pub instances: Vec<Instance>,
    pub instances_scroll_state: ListState,
    pub current_instance: Instance,


}

impl App {
    pub async fn new() -> Self {
        Self {

            meili_client: get_client(),

            selected_tab: AppTabs::DocumentsTab, // check if there is an instance, if not, switch to instances tab
            should_quit: false,
            should_redraw: true,
            // status: "Loading documents...".to_string(),
            app_mode: AppMode::Normal,

            documents: api::get_documents().await,
            documents_scroll_state: ListState::default(),

            last_refreshed: format!("{}", chrono::Local::now().format("%H:%M:%S")),

            // search parameters
            query: "".to_string(),
            filter_query: "".to_string(),
            sort_query: "".to_string(),

            cursor_position: 0,
            filter_cursor_position: 0,
            sort_cursor_position: 0,

            current_search_form: SearchForm::Query,
        

            tasks: api::get_tasks().await,
            task_scroll_state: ListState::default(),

            // index related
            indices: api::get_all_indices().await,
            indices_scroll_state: ListState::default(),
            current_index: None,

            all_index_settings: get_all_index_settings().await,

            // instances related
            instances: retrieve_instances_from_file(),
            instances_scroll_state: ListState::default(),
            //temp
            current_instance: Instance {
                id: "1".to_string(), 
                name: "Movies Production".to_string(),
                host: "localhost".to_string(),
                primary_key: "".to_string()
            },

        }
    }

    pub async fn search_documents(&mut self) {

        //check if an index is selected before searching
        match &self.current_index {
            Some(index) => {
                self.documents = api::search_documents(&self.query, &self.filter_query, &self.sort_query, &index).await;
                self.documents_scroll_state = ListState::default();
                self.update_last_refreshed();
            }
            None => {
                //todo: show a message to the user
            }
        }
    }

    pub fn get_current_document_info(&self) -> String {
        //get the current document info from the vector using the list state 
        let selected_document = match self.documents_scroll_state.selected() {
            Some(index) => index,
            None => {
                return "No document selected".to_string();
            }
        };

        let document = &self.documents[selected_document];
        let pretty_json = match serde_json::to_string_pretty(document) {
            Ok(json) => json,
            Err(_) => format!("{:#?}", document),
        };
        
        pretty_json
    }

    pub fn get_current_task_info(&self) -> String {
        
        //get the current task info from the vector using the list state 
        let selected_task = match self.task_scroll_state.selected() {
            Some(index) => index,
            None => {
                return "No task selected".to_string();
            }
        };

        
        let task = &self.tasks[selected_task];
        // todo: custom formatter
        let debug_print = format!("{:#?}", task);
        debug_print
        // let json = serde_json::to_string_pretty(&debug_print)
        //     .unwrap_or_else(|_| debug_print);

        // json
    }

    pub fn get_current_index_settings(&self) -> String {
        //get the current index info from the vector using the list state 
        let selected_index = match self.indices_scroll_state.selected() {
            Some(index) => index,
            None => {
                return "No index selected".to_string();
            }
        };

        let index_settigs = &self.all_index_settings[selected_index];
        let pretty_json = match serde_json::to_string_pretty(index_settigs) {
            Ok(json) => json,
            Err(_) => format!("{:#?}", index_settigs),
        };
        
        pretty_json
    }

    fn update_last_refreshed(&mut self) {
        let time_now = chrono::Local::now();
        self.last_refreshed = format!("{}", time_now.format("%H:%M:%S"));
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
            AppTabs::DocumentsTab => scroll_state_incrementer(&mut self.documents_scroll_state, &self.documents.len() as &usize),
            AppTabs::TasksTab => scroll_state_incrementer(&mut self.task_scroll_state, &self.tasks.len() as &usize),
            AppTabs::IndicesTab => scroll_state_incrementer(&mut self.indices_scroll_state, &self.indices.len() as &usize),
            AppTabs::InstancesTab => scroll_state_incrementer(&mut self.instances_scroll_state, &self.instances.len() as &usize),
        }
    }

    pub fn decrement_scroll_state(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => scroll_state_decrementer(&mut self.documents_scroll_state, &self.documents.len() as &usize),
            AppTabs::TasksTab => scroll_state_decrementer(&mut self.task_scroll_state, &self.tasks.len() as &usize),
            AppTabs::IndicesTab => scroll_state_decrementer(&mut self.indices_scroll_state, &self.indices.len() as &usize),
            AppTabs::InstancesTab => scroll_state_decrementer(&mut self.instances_scroll_state, &self.instances.len() as &usize),
        }
    }


    // this is used to change current index or instance depending on the current tab
    pub fn select_item(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => {}
            AppTabs::TasksTab => {}
            AppTabs::IndicesTab => {
                let selected_index = match self.indices_scroll_state.selected() {
                    Some(index) => index,
                    None => {
                        return;
                    }
                };
                self.current_index = Some(self.indices[selected_index].clone());
            }
            AppTabs::InstancesTab => {}
        }
    }

}


// 🦀 second impl block for the search/input functionality
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


    fn general_delete_char(query: &mut String, cursor_position: usize){
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
            let new_query: std::string::String = before_char_to_delete.chain(after_char_to_delete).collect();
            query.clear();
            query.push_str(&new_query);
        }
            
    }


    pub fn delete_char(&mut self) {

        match self.current_search_form {
            SearchForm::Query => {
                Self::general_delete_char(&mut self.query, self.cursor_position);
                self.move_cursor_left();
            }
            SearchForm::Filter => {
                Self::general_delete_char(&mut self.filter_query, self.filter_cursor_position);
                self.move_cursor_left();
            }
            SearchForm::Sort => {
                Self::general_delete_char(&mut self.sort_query, self.sort_cursor_position);
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
            SearchForm::Sort => new_cursor_pos.clamp(0, self.sort_query.chars().count())
        }
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
    
}
