use ratatui::style::Color;

use crate::api::{bulk_delete_by_filter, delete_document};

use super::app::{App, AppTabs};

// for action mode
impl App {
    pub async fn delete_item(&mut self) {
        match self.selected_tab {
            AppTabs::DocumentsTab => {
                // delete the selected document

                // todo: how do we know the correct index

                let index = match &self.current_index {
                    Some(index) => index,
                    None => {
                        return;
                    }
                };

                let selected_document_id = match self.get_current_document_id() {
                    Some(id) => id,
                    None => {
                        return;
                    }
                };

                delete_document(&index.uid, selected_document_id, &self.meili_client).await;
                //todo: get result from above
                self.show_toast("Item deleted".to_string(), Color::Green);
                self.refresh_current_items().await;
            }
            AppTabs::TasksTab => {
                // cancel the selected task
            }
            AppTabs::IndicesTab => {
                // no op needed, for now
            }
            AppTabs::InstancesTab => {
                // remove the instance? Probably not.
            }
        }
    }


        // bulk delete
    pub async fn bulk_delete_by_filter(&mut self) {
        if self.selected_tab != AppTabs::DocumentsTab {
            return;
        }

        let filter = self.filter_query.clone();
        if filter.is_empty() {
            self.show_toast("No filter is present".to_string(), Color::Yellow);
            return;
        }

        match &self.current_index {
            Some(index) => {
                match bulk_delete_by_filter(index, &filter).await {
                    Ok(_) => {
                        self.show_toast("Items deleted".to_string(), Color::Green);
                    }
                    Err(_) => {
                        self.show_toast("Error deleting by filter".to_string(), Color::Red);
                    }
                }

                self.refresh_current_items().await;
            }
            None => {
                //inform the user that no index is selected
                self.show_toast("No index is selected".to_string(), Color::Red);
            }
        }
    }




}