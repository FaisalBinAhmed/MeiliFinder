use meilisearch_sdk::{EnqueuedTask, FailedTask, ProcessingTask, SucceededTask, Task, TaskType};
use ratatui::{style::{Color, Style}, text::Span};



pub fn get_task_type_name(task_type: &TaskType) -> String {
    match task_type {
        TaskType::DocumentAdditionOrUpdate { details } => "Document Addition Or Update".to_string(),
        TaskType::DocumentDeletion { details } => "Document Deletion".to_string(),
        TaskType::SettingsUpdate { details } => "Settings Update".to_string(),
        TaskType::DumpCreation { details } => "Dump Creation".to_string(),
        TaskType::IndexCreation { details } => "Index Creation".to_string(),
        TaskType::IndexUpdate { details } => "Index Update".to_string(),
        TaskType::TaskDeletion { details } => "Task Deletion".to_string(),
        TaskType::Customs => "Customs".to_string(),
        TaskType::TaskCancelation { details } => "Task Cancelation".to_string(),
        TaskType::SnapshotCreation { details } => "Snapshot Creation".to_string(),
        TaskType::IndexDeletion { details } => "Index Deletion".to_string(),
        TaskType::IndexSwap { details } => "Index Swap".to_string(),
    }
}
