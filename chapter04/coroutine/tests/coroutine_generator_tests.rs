use coroutine::{coroutine_worker, run_coroutines};
use futures::StreamExt;

#[tokio::test]
async fn test_single_coroutine() {
    let mut stream = coroutine_worker(1, 3);
    let mut results = Vec::new();

    while let Some(msg) = stream.next().await {
        results.push(msg);
    }

    assert_eq!(results, vec![
        "Worker 1 yielded step 1",
        "Worker 1 yielded step 2",
        "Worker 1 yielded step 3",
    ]);
}

#[tokio::test]
async fn test_multiple_coroutines() {
    let results = run_coroutines(2, 2).await;

    // Each worker yields 2 steps, so total = 4 results
    assert_eq!(results.len(), 4);

    // The order is not deterministic, but all messages must exist.
    let expected = vec![
        "Worker 1 yielded step 1".to_string(),
        "Worker 1 yielded step 2".to_string(),
        "Worker 2 yielded step 1".to_string(),
        "Worker 2 yielded step 2".to_string(),
    ];

    for msg in expected {
        assert!(results.contains(&msg));
    }
}
