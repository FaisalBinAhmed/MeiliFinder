use meilisearch_sdk::{client, Client, DocumentsQuery, DocumentsResults, Task, TasksResults};

pub async fn get_tasks() -> Vec<Task> {
    // todo: get the Instance info from the app state
    let client = Client::new("http://localhost:7700", Some("ZL4dOFgqygBrAGPapWs2LdgTSdveZ8qdsWTyBlyF9-M"));
    let tasks_result = client.get_tasks().await;

    match tasks_result {
        Ok(tasks) => return tasks.results,
        Err(_) => return  vec![]
    }

}

pub async fn get_documents() {
        let client = Client::new("http://localhost:7700", Some("ZL4dOFgqygBrAGPapWs2LdgTSdveZ8qdsWTyBlyF9-M"));

    let movies = client.index("movies");

    let documents = DocumentsQuery::new(&movies)
  .with_limit(2)
  .execute::<serde_json::Value>()
  .await
  .unwrap();

//   return documents.results;
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