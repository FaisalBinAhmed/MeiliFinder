use meilisearch_sdk::{client, documents, Client, DocumentsQuery, DocumentsResults, SearchResults, Task, TasksResults};
use serde_json::Value;


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


pub async fn search_documents(query: &str, filter: &str, sort: &str, client: &Client) -> Vec<Value> {

    let search_result: Result<SearchResults<Value>, _> = client
        .index("movies")
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