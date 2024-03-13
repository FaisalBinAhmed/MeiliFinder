use meilisearch_sdk::{
    client, documents, Client, DocumentDeletionQuery, DocumentsQuery, DocumentsResults, Index, IndexesQuery, IndexesResults, SearchResults, Settings, Task, TaskInfo, TasksResults
};
use serde_json::Value;

use crate::{app::{ResultMetadata, Toast}, utilities::config_handler::retrieve_instances_from_file};

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

// for now
pub fn get_client() -> Client {
    Client::new(
        "http://localhost:7700",
        Some("ZL4dOFgqygBrAGPapWs2LdgTSdveZ8qdsWTyBlyF9-M"),
    )
}

pub async fn search_documents(query: &str, filter: &str, index: &Index) -> (Vec<Value>, ResultMetadata){

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
            
            return (documents, ResultMetadata{
                estimated_total_hits: search_result.estimated_total_hits.unwrap_or(0),
                hits: search_result.hits.len() as usize,
                processing_time_ms: search_result.processing_time_ms as usize
            })
        },
        Err(_) => (vec![], ResultMetadata::default())
    };

    document_results

}

// I don't like this duplication, but I couldn't make the other function work with "" as default sort query
pub async fn search_documents_with_sort(query: &str, filter: &str, sort: &str, index: &Index) -> (Vec<Value>, ResultMetadata) {

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
            
            return (documents, ResultMetadata{
                estimated_total_hits: search_result.estimated_total_hits.unwrap_or(0),
                hits: search_result.hits.len() as usize,
                processing_time_ms: search_result.processing_time_ms as usize
            })
        },
        Err(_) => (vec![], ResultMetadata::default())
    };

    document_results
}

pub async fn get_tasks() -> Vec<Task> {
    // todo: get the Instance info from the app state
    let client = get_client(); //temp
    let tasks_result = client.get_tasks().await;

    match tasks_result {
        Ok(tasks) => return tasks.results,
        Err(_) => return vec![],
    }
}

pub async fn get_documents(index: &str) -> (Vec<Value>, ResultMetadata) {
    let client = get_client(); //temp

    let movies = client.index(index);

    let documents = DocumentsQuery::new(&movies)
        .with_limit(20)
        .execute::<Value>()
        .await;

    match documents {
        Ok(documents) => return (documents.results.clone(), ResultMetadata{
            estimated_total_hits: documents.total as usize,
            hits: documents.results.len() as usize,
            processing_time_ms: 0
        }),
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

pub async fn get_task_by_id(task_id: u32) -> Option<String> {
    let master_key = "MASTER_KEY";

    let url = format!("http://localhost:7700/tasks/{}", task_id);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", master_key),
        )
        .send()
        .await;

    match response {
        Ok(response) => {
            let task = response.text().await.unwrap_or_else(|_| String::new());
            return Some(task);
        }
        Err(_) => return None,
    }
}

pub async fn get_all_indices() -> Vec<Index> {
    let client = get_client();

    let indices_result: Result<IndexesResults, _> = IndexesQuery::new(&client)
        // .with_limit(3)
        .execute()
        .await;

    match indices_result {
        Ok(indices_result) => return indices_result.results,
        Err(_) => return vec![],
    }
}

pub async fn get_all_index_settings() -> Vec<Settings> {
    let indices = get_all_indices().await;

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

pub async fn delete_document(index_uid: &str, document_id: &str) {
    let client = get_client();

    let taskinfo_result = client.index(index_uid).delete_document(document_id).await;

    match taskinfo_result {
        Ok(_taskinfo) => (),
        Err(_) => (),
    }
}


pub async fn bulk_delete_by_filter(index: &Index, filter: &str) -> Result<TaskInfo, meilisearch_sdk::Error> {
    let task = DocumentDeletionQuery::new(&index)
        .with_filter(filter)
        .execute::<TaskInfo>() // Add type annotation here
        .await;

    task

    // match task {
    //     Ok(task) => println!("Task: {:?}", task),
    //     Err(e) => println!("Error: {:?}", e),
    // }
}