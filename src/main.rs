use std::sync::Arc;

async fn worker(client:  Arc<reqwest::Client>, id: u32) -> Result<(), Box<dyn std::error::Error>>{
    let body = client
    .get("https://jsonplaceholder.typicode.com/posts/".to_string() + &id.to_string())
    .send()
    .await?
    .text()
    .await?;

    // print the response body length
    println!("Response from {}: {:?}", id, body.len());
    Ok(())
}

#[tokio::main]
async fn main() {

    // create an 'Atomically Reference Counted' reqwest client pointer
    let arc_client = Arc::new(reqwest::Client::new());

    // vector to hold thread handles
    let mut thread_handles = Vec::new();

    println!("Started...");

    // create 10 tokio threads 
    for i in 1..11 {
        let _client = arc_client.clone();
        thread_handles.push(tokio::spawn(async move {
            let result = worker(_client, i).await;
        }));
    }
    
    // 'join' the threads
    for th in thread_handles {
        let result = th.await;
    }

    println!("Completed...");
}