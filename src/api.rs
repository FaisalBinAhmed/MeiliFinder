use meilisearch_sdk::{
    Client, DocumentDeletionQuery, DocumentsQuery, Index, IndexesQuery, IndexesResults,
    SearchResults, Settings, Task, TaskInfo,
};
use serde_json::Value;

use crate::{app::app::ResultMetadata, utilities::config_handler::retrieve_instances_from_file};

pub struct TaskId {
    pub id: u32,
}

impl AsRef<u32> for TaskId {
    fn as_ref(&self) -> &u32 {
        &self.id
    }
}

pub fn get_inital_client() -> Option<Client> {
    let inital_instance = retrieve_instances_from_file().first().cloned();

    match inital_instance {
        Some(instance) => {
            let client = Client::new(instance.host, Some(instance.primary_key));
            Some(client)
        }
        None => None,
    }
}

pub async fn search_documents(
    query: &str,
    filter: &str,
    index: &Index,
) -> (Vec<Value>, ResultMetadata) {
    let search_result: Result<SearchResults<Value>, _> = index
        .search()
        .with_query(query)
        .with_filter(filter)
        .execute()
        .await;

    let document_results = match search_result {
        Ok(search_result) => {
            let documents = search_result
                .hits
                .iter()
                .map(|hit| hit.result.clone())
                .collect();

            return (
                documents,
                ResultMetadata {
                    estimated_total_hits: search_result.estimated_total_hits.unwrap_or(0),
                    hits: search_result.hits.len() as usize,
                    processing_time_ms: search_result.processing_time_ms as usize,
                },
            );
        }
        Err(_) => (vec![], ResultMetadata::default()),
    };

    document_results
}

// I don't like this duplication, but I couldn't make the other function work with "" as default sort query
pub async fn search_documents_with_sort(
    query: &str,
    filter: &str,
    sort: &str,
    index: &Index,
) -> (Vec<Value>, ResultMetadata) {
    let search_result: Result<SearchResults<Value>, _> = index
        .search()
        .with_query(query)
        .with_filter(filter)
        .with_sort(&[sort])
        .execute()
        .await;

    let document_results = match search_result {
        Ok(search_result) => {
            let documents = search_result
                .hits
                .iter()
                .map(|hit| hit.result.clone())
                .collect();

            return (
                documents,
                ResultMetadata {
                    estimated_total_hits: search_result.estimated_total_hits.unwrap_or(0),
                    hits: search_result.hits.len() as usize,
                    processing_time_ms: search_result.processing_time_ms as usize,
                },
            );
        }
        Err(_) => (vec![], ResultMetadata::default()),
    };

    document_results
}

pub async fn get_tasks(client: &Option<Client>) -> Vec<Task> {
    let client = match client {
        Some(client) => client,
        None => return vec![],
    };

    let tasks_result = client.get_tasks().await;

    match tasks_result {
        Ok(tasks) => return tasks.results,
        Err(_) => return vec![],
    }
}

pub async fn get_documents(index: &str, client: &Option<Client>) -> (Vec<Value>, ResultMetadata) {
    let client = match client {
        Some(client) => client,
        None => return (vec![], ResultMetadata::default()),
    };

    let movies = client.index(index);

    let documents = DocumentsQuery::new(&movies)
        .with_limit(20)
        .execute::<Value>()
        .await;

    match documents {
        Ok(documents) => {
            return (
                documents.results.clone(),
                ResultMetadata {
                    estimated_total_hits: documents.total as usize,
                    hits: documents.results.len() as usize,
                    processing_time_ms: 0,
                },
            )
        }
        Err(_) => return (vec![], ResultMetadata::default()),
    }
}

pub async fn get_task_by_id_meili(task_id: TaskId, client: &Client) -> Option<Task> {
    let task = client.get_task(task_id).await;

    match task {
        Ok(task) => return Some(task),
        Err(_) => return None,
    }
}

pub async fn get_all_indices(client: &Option<Client>) -> Vec<Index> {
    let client = match client {
        Some(client) => client,
        None => return vec![],
    };

    let indices_result: Result<IndexesResults, _> = IndexesQuery::new(&client)
        // .with_limit(3)
        .execute()
        .await;

    match indices_result {
        Ok(indices_result) => return indices_result.results,
        Err(_) => return vec![],
    }
}

pub async fn get_all_index_settings(client: &Option<Client>) -> Vec<Settings> {
    let indices = get_all_indices(client).await;

    let mut settings: Vec<Settings> = vec![];

    for index in indices {
        let index_settings = index.get_settings().await;
        match index_settings {
            Ok(index_settings) => settings.push(index_settings),
            Err(_) => continue,
        }
    }

    settings
}

pub async fn delete_document(index_uid: &str, document_id: &str, client: &Option<Client>) {
    let client = match client {
        Some(client) => client,
        None => return,
    };

    let taskinfo_result = client.index(index_uid).delete_document(document_id).await;

    match taskinfo_result {
        Ok(_taskinfo) => (),
        Err(_) => (),
    }
}

pub async fn bulk_delete_by_filter(
    index: &Index,
    filter: &str,
) -> Result<TaskInfo, meilisearch_sdk::Error> {
    let task = DocumentDeletionQuery::new(&index)
        .with_filter(filter)
        .execute::<TaskInfo>() // Add type annotation here
        .await;

    task
}
