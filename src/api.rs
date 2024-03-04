use meilisearch_sdk::{client, documents, Client, DocumentsQuery, DocumentsResults, Index, IndexesQuery, IndexesResults, SearchResults, Settings, Task, TaskInfo, TasksResults};
use serde_json::Value;

use crate::app;


pub struct TaskId {
    pub id: u32,
}

impl AsRef<u32> for TaskId {
    fn as_ref(&self) -> &u32 {
        &self.id
    }
}

// for now
pub fn get_client() -> Client {
    Client::new("http://localhost:7700", Some("ZL4dOFgqygBrAGPapWs2LdgTSdveZ8qdsWTyBlyF9-M"))
}


pub async fn search_documents(query: &str, filter: &str, sort: &str, index: &Index) -> Vec<Value> {


    let search_result: Result<SearchResults<Value>, _> = index
        .search()
        .with_query(query)
        .with_filter(filter)
        // .with_sort(&[sort]) //todo: not working: add default value
        .execute()
        .await;

    let documents: Vec<Value> = match search_result {
        Ok(search_result) => search_result.hits.iter().map(|hit| hit.result.clone()).collect(),
        Err(_) => vec![]
    };

    documents

}


pub async fn get_tasks() -> Vec<Task> {
    // todo: get the Instance info from the app state
    let client = get_client(); //temp
    let tasks_result = client.get_tasks().await;

    match tasks_result {
        Ok(tasks) => return tasks.results,
        Err(_) => return  vec![]
    }

}

pub async fn get_documents() -> Vec<Value> {

    let client = get_client(); //temp

    let movies = client.index("movies");

    let documents = DocumentsQuery::new(&movies)
  .with_limit(20)
  .execute::<Value>()
  .await;

  match documents {
      Ok(documents) => return documents.results,
      Err(_) => return vec![]
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
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", master_key))
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
        Err(_) => return vec![]
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

   let taskinfo_result = client
  .index(index_uid)
  .delete_document(document_id)
  .await;

    match taskinfo_result {
        Ok(_taskinfo) => (),
        Err(_) => ()
    }


    
}