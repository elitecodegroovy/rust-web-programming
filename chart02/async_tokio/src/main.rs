use tokio::time::Duration;

async fn fetch_data() -> Result<String, reqwest::Error> { 
    tokio::time::sleep(Duration::from_secs(2)).await; 
    Ok::<String, _>("Loading data successfully".to_owned()) 
}

async fn process_data(data: String) { 
    println!("Processing: {}", data); 
}

async fn main_async_workflow() { // ⑥
    let fetch_task = tokio::spawn(fetch_data()); 
    let process_task = tokio::spawn(process_data("Sample Data".to_owned())); 
    let fetch_result = fetch_task.await.expect("Failed to fetch data"); 
    let _ = process_task.await; 
    println!("Fetch Result: {:?}", fetch_result); 

    println!("successfully exit app!")
}


#[tokio::main] 
async fn main() {
    main_async_workflow().await; 
}
