use coroutine::{worker, run_workers};

#[tokio::test]
async fn test_single_worker() {
    let results = worker(1, 3).await;

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "Worker 1 finished step 1");
    assert_eq!(results[1], "Worker 1 finished step 2");
    assert_eq!(results[2], "Worker 1 finished step 3");
}

#[tokio::test]
async fn test_multiple_workers() {
    let results = run_workers(2, 2).await;

    // Each worker runs 2 steps, so total = 4 results
    assert_eq!(results.len(), 4);

    // The order is not guaranteed because tasks are concurrent,
    // but all expected messages must exist.
    let expected = vec![
        "Worker 1 finished step 1".to_string(),
        "Worker 1 finished step 2".to_string(),
        "Worker 2 finished step 1".to_string(),
        "Worker 2 finished step 2".to_string(),
    ];

    for msg in expected {
        assert!(results.contains(&msg));
    }
}
