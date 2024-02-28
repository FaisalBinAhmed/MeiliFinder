use meilisearch_sdk::{client, Client, DocumentsQuery, DocumentsResults, Task, TasksResults};
use serde_json::Value;


pub struct TaskId {
    pub id: u32,
}

impl AsRef<u32> for TaskId {
    fn as_ref(&self) -> &u32 {
        &self.id
    }
}

pub async fn get_tasks() -> Vec<Task> {
    // todo: get the Instance info from the app state
    let client = Client::new("http://localhost:7700", Some("ZL4dOFgqygBrAGPapWs2LdgTSdveZ8qdsWTyBlyF9-M"));
    let tasks_result = client.get_tasks().await;

    match tasks_result {
        Ok(tasks) => return tasks.results,
        Err(_) => return  vec![]
    }

}

pub async fn get_documents() -> Vec<Value> {
        let client = Client::new("http://localhost:7700", Some("ZL4dOFgqygBrAGPapWs2LdgTSdveZ8qdsWTyBlyF9-M"));

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

pub async fn get_task_by_id_meili(task_id: TaskId) -> Option<Task> {
    let client = Client::new("http://localhost:7700", Some("ZL4dOFgqygBrAGPapWs2LdgTSdveZ8qdsWTyBlyF9-M"));

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