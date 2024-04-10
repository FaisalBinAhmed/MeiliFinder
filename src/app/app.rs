use meilisearch_sdk::{Client, Index, Settings};
use ratatui::{style::Color, widgets::ListState};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    api::{self, get_all_index_settings, get_documents, get_inital_client},
    event::Event,
    utilities::{
        config_handler::retrieve_instances_from_file,
        helpers::{get_initial_documents, get_initial_index, get_initial_instance},
        scrolling_handler::{scroll_state_decrementer, scroll_state_incrementer},
    },
};

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
    Preview,
    Delete,
}

#[derive(PartialEq)]
pub enum DeleteType {
    Single,
    Bulk,
}

#[derive(Debug, PartialEq)]
pub enum SearchForm {
    Query,
    Filter,
    Sort,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    pub id: String,          // unique id
    pub name: String,        // name of the instance, optional
    pub host: String,        // host url of the instance
    pub primary_key: String, // primary api key to access the instance
}

#[derive(Default)]
pub struct ResultMetadata {
    pub hits: usize,
    pub estimated_total_hits: usize,
    pub processing_time_ms: usize,
}

pub struct Toast {
    pub message: String,
    pub color: Color,
}

pub struct App {
    pub meili_client: Option<Client>,

    pub selected_tab: AppTabs,
    pub should_quit: bool,

    pub should_redraw: bool,
    pub app_mode: AppMode,

    pub documents: Vec<serde_json::Value>,
    pub documents_scroll_state: ListState,

    pub current_result_metadata: ResultMetadata,

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
    // this is used to store all index settings so we can display them in the preview
    pub all_index_settings: Vec<Settings>,

    pub instances: Vec<Instance>,
    pub instances_scroll_state: ListState,

    pub current_instance: Option<Instance>,

    // toast related
    pub toast: Option<Toast>,
    pub sender: UnboundedSender<Event>,

    // delete mode related
    pub delete_type: DeleteType,

    // preview related
    pub preview_scroll_state: ListState,
}

impl App {
    pub async fn new(sender: UnboundedSender<Event>, client: Option<Client>) -> Self {
        Self {
            meili_client: get_inital_client(), // should be updated when the user selects an instance

            selected_tab: AppTabs::DocumentsTab, // check if there is an instance, if not, switch to instances tab
            should_quit: false,
            should_redraw: true,

            app_mode: AppMode::Normal,

            documents: get_initial_documents(&client).await,
            documents_scroll_state: ListState::default(),

            current_result_metadata: ResultMetadata::default(),

            last_refreshed: format!("{}", chrono::Local::now().format("%H:%M:%S")),

            // search parameters
            query: "".to_string(),
            filter_query: "".to_string(),
            sort_query: "".to_string(),

            cursor_position: 0,
            filter_cursor_position: 0,
            sort_cursor_position: 0,

            current_search_form: SearchForm::Query,

            tasks: api::get_tasks(&client).await,
            task_scroll_state: ListState::default(),

            // index related
            indices: api::get_all_indices(&client).await,
            indices_scroll_state: ListState::default(),
            current_index: get_initial_index(&client).await,

            all_index_settings: get_all_index_settings(&client).await,

            // instances related
            instances: retrieve_instances_from_file(),
            instances_scroll_state: ListState::default(),
            current_instance: get_initial_instance(),

            // toast related
            toast: None,
            sender: sender,

            // delete mode related
            delete_type: DeleteType::Single,

            // preview related
            preview_scroll_state: ListState::default(),
        }
    }

    pub async fn search_documents(&mut self) {
        //check if an index is selected before searching
        match &self.current_index {
            Some(index) => {
                if self.sort_query.is_empty() {
                    (self.documents, self.current_result_metadata) =
                        api::search_documents(&self.query, &self.filter_query, &index).await;
                } else {
                    (self.documents, self.current_result_metadata) =
                        api::search_documents_with_sort(
                            &self.query,
                            &self.filter_query,
                            &self.sort_query,
                            &index,
                        )
                        .await;
                }
                self.documents_scroll_state = ListState::default();
                self.update_last_refreshed();
            }
            None => {
                //todo: show a message to the user
            }
        }
    }

    pub fn get_item_to_delete_info(&self) -> String {
        match self.delete_type {
            DeleteType::Single => self.get_current_document_info(),
            DeleteType::Bulk => self.get_items_by_filter_info(),
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

    pub fn get_items_by_filter_info(&self) -> String {
        let multiline_string = format!(
r#" ⚠️  DELETING ITEMS IN BULK ⚠️

 Current Instance: {}

 Selected Index: {}

 Current Filter Query: {}

 You are about to delete {} items.

 Are you sure?"#,
           self.get_current_instance_info(), self.get_current_index_name(), self.filter_query, self.current_result_metadata.estimated_total_hits
        );

        multiline_string
    }


    pub fn get_current_instance_info(&self) -> String {
        let instance = match &self.current_instance {
            Some(instance) => instance.name.clone(),
            None => {
                return "No instance selected".to_string();
            }
        };

        instance
    }

    pub fn get_current_document_id(&self) -> Option<String> {
        //get the current document info from the vector using the list state
        let selected_document_index = self.documents_scroll_state.selected()?;

        // get current index
        let index = &self.current_index.clone()?;

        // get primary key of the index
        let primary_key = &index.primary_key.clone()?;

        // then we can get the document id from the primary key, the value is the document id
        let document = &self.documents[selected_document_index];

        let id = document.get(primary_key)?.to_string();

        Some(id)
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
    }

    pub fn get_current_index_name(&self) -> String {
        let current_index: String = match &self.current_index {
        Some(index) => index.uid.clone(),
        None => "No index selected".to_string(),
    };
    current_index
    }

    pub fn get_current_index_settings(&self) -> String {
        //get the current index info from the vector using the list state
        let selected_index = match self.indices_scroll_state.selected() {
            Some(index) => index,
            None => {
                return "No index selected".to_string();
                // todo: check current_index as well
            }
        };

        let index_settigs = &self.all_index_settings[selected_index];
        let pretty_json = match serde_json::to_string_pretty(index_settigs) {
            Ok(json) => json,
            Err(_) => format!("{:#?}", index_settigs), // debug print for now
        };

        pretty_json
    }

    pub async fn refresh_current_items(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => {
                self.search_documents().await;
            }
            AppTabs::TasksTab => {
                self.tasks = api::get_tasks(&self.meili_client).await;
            }
            AppTabs::IndicesTab => {
                self.indices = api::get_all_indices(&self.meili_client).await;
                self.all_index_settings = get_all_index_settings(&self.meili_client).await;
            }
            AppTabs::InstancesTab => {
                // self.instances = retrieve_instances_from_file();
            }
        }
        self.update_last_refreshed();
    }

    fn update_last_refreshed(&mut self) {
        let time_now = chrono::Local::now();
        self.last_refreshed = format!("{}", time_now.format("%H:%M:%S"));
    }

    pub fn reset_all_queries(&mut self) {
        self.query.clear();
        self.cursor_position = 0;
        self.filter_query.clear();
        self.filter_cursor_position = 0;
        self.sort_query.clear();
        self.sort_cursor_position = 0;
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
            AppTabs::DocumentsTab => scroll_state_incrementer(
                &mut self.documents_scroll_state,
                &self.documents.len() as &usize,
            ),
            AppTabs::TasksTab => {
                scroll_state_incrementer(&mut self.task_scroll_state, &self.tasks.len() as &usize)
            }
            AppTabs::IndicesTab => scroll_state_incrementer(
                &mut self.indices_scroll_state,
                &self.indices.len() as &usize,
            ),
            AppTabs::InstancesTab => scroll_state_incrementer(
                &mut self.instances_scroll_state,
                &self.instances.len() as &usize,
            ),
        }
    }

    pub fn decrement_scroll_state(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => scroll_state_decrementer(
                &mut self.documents_scroll_state,
                &self.documents.len() as &usize,
            ),
            AppTabs::TasksTab => {
                scroll_state_decrementer(&mut self.task_scroll_state, &self.tasks.len() as &usize)
            }
            AppTabs::IndicesTab => scroll_state_decrementer(
                &mut self.indices_scroll_state,
                &self.indices.len() as &usize,
            ),
            AppTabs::InstancesTab => scroll_state_decrementer(
                &mut self.instances_scroll_state,
                &self.instances.len() as &usize,
            ),
        }
    }

    pub fn increment_preview_scroll_state(&mut self) {

        //todo: move to state
        let current_document_lines = self.get_current_document_info().lines().count();

        scroll_state_incrementer(
            &mut self.preview_scroll_state,
            &current_document_lines
        );
    }

    pub fn decrement_preview_scroll_state(&mut self) {
        let current_document_lines = self.get_current_document_info().lines().count();
        scroll_state_decrementer(
            &mut self.preview_scroll_state,
            &current_document_lines
        );
    }

    // this is used to change current index or instance depending on the current tab
    pub async fn select_item(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => {}
            AppTabs::IndicesTab => {
                let selected_index = match self.indices_scroll_state.selected() {
                    Some(index) => index,
                    None => {
                        return;
                    }
                };
                self.current_index = Some(self.indices[selected_index].clone());
                (self.documents, self.current_result_metadata) =
                get_documents(&self.indices[selected_index].uid, &self.meili_client).await;
                self.documents_scroll_state = ListState::default();

            }
            AppTabs::TasksTab => {}
            AppTabs::InstancesTab => {
                let selected_instance = match self.instances_scroll_state.selected() {
                    Some(index) => index,
                    None => {
                        return;
                    }
                };
                self.current_instance = Some(self.instances[selected_instance].clone());

                self.update_client_with_current_instance();

                self.tasks = api::get_tasks(&self.meili_client).await;
                self.indices = api::get_all_indices(&self.meili_client).await;
                self.documents = get_initial_documents(&self.meili_client).await;

                // todo: update other info
            }
        }
    }

    pub fn update_client_with_current_instance(&mut self) {
        let current_instance = self.current_instance.clone();
        match current_instance {
            Some(instance) => {
                self.meili_client = Some(Client::new(instance.host, Some(instance.primary_key)));
            }
            None => {
                self.meili_client = None;
            }
        }
    }

}
