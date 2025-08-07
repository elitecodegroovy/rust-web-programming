use tokio::time::Duration;

async fn fetch_data() -> Result<String, reqwest::Error> { // ①
    tokio::time::sleep(Duration::from_secs(2)).await; // ②
    Ok::<String, _>("Data fetched successfully".to_owned()) // ③
}

async fn process_data(data: String) { // ④
    println!("Processing: {}", data); // ⑤
}

async fn main_async_workflow() { // ⑥
    let fetch_task = tokio::spawn(fetch_data()); // ⑦
    let process_task = tokio::spawn(process_data("Sample Data".to_owned())); // ⑧
    let fetch_result = fetch_task.await.expect("Failed to fetch data"); // ⑨
    let _ = process_task.await; // ⑩
    println!("Fetch Result: {:?}", fetch_result); // ⑪

    println!("successfully exit app!")
}


#[tokio::main] // ⑫
async fn main() {
    main_async_workflow().await; // ⑬
}
// Output
// Processing: Sample Data
// Fetch Result: Ok(“Data fetched successfully”)