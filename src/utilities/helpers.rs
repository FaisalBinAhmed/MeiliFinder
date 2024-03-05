use meilisearch_sdk::{Index, TaskType};
use serde_json::Value;

use crate::api;

pub fn get_task_type_name(task_type: &TaskType) -> String {
    match task_type {
        TaskType::DocumentAdditionOrUpdate { details: _ } => "Document Addition Or Update".to_string(),
        TaskType::DocumentDeletion { details: _ } => "Document Deletion".to_string(),
        TaskType::SettingsUpdate { details: _ } => "Settings Update".to_string(),
        TaskType::DumpCreation { details: _ } => "Dump Creation".to_string(),
        TaskType::IndexCreation { details: _ } => "Index Creation".to_string(),
        TaskType::IndexUpdate { details: _ } => "Index Update".to_string(),
        TaskType::TaskDeletion { details: _ } => "Task Deletion".to_string(),
        TaskType::TaskCancelation { details: _ } => "Task Cancelation".to_string(),
        TaskType::SnapshotCreation { details: _ } => "Snapshot Creation".to_string(),
        TaskType::IndexDeletion { details: _ } => "Index Deletion".to_string(),
        TaskType::IndexSwap { details: _ } => "Index Swap".to_string(),
        TaskType::Customs => "Custom Task".to_string(),
    }
}


pub async fn get_initial_index() -> Option<Index> {
        api::get_all_indices().await.first().cloned()
    }

pub async fn get_initial_documents() -> Vec<Value> {
    let index = match get_initial_index().await {
        Some(index) => index,
        None => return vec![],
    };

    api::get_documents(&index.uid).await

}