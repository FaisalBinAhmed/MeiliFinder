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
    // pub selected_instance: Instance,

}

impl App {
    pub async fn new() -> Self {
        Self {
            selected_tab: AppTabs::DocumentsTab, // check if there is an instance, if not, switch to instances tab
            should_quit: false,
            should_redraw: true,
            status: "Loading documents...".to_string(),
            app_mode: AppMode::Normal,
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
    
}
