use tokio::time::{sleep, Duration};

use async_stream::stream;
use futures::{Stream, StreamExt};
use std::pin::Pin;

/// A simple coroutine-like async function that simulates work.
pub async fn worker(id: usize, steps: usize) -> Vec<String> {
    let mut results = Vec::new();

    for step in 1..=steps {
        // simulate async "yield"
        sleep(Duration::from_millis(10)).await;

        let msg = format!("Worker {} finished step {}", id, step);
        results.push(msg);
    }

    results
}

/// Run multiple workers concurrently like coroutines.
pub async fn run_workers(num_workers: usize, steps: usize) -> Vec<String> {
    let mut handles = Vec::new();

    for i in 1..=num_workers {
        handles.push(tokio::spawn(worker(i, steps)));
    }

    let mut all_results = Vec::new();
    for handle in handles {
        let res = handle.await.unwrap();
        all_results.extend(res);
    }

    all_results
}


/// A coroutine-like async generator.
/// Yields a sequence of strings, simulating async work.
pub fn coroutine_worker(id: usize, steps: usize) -> Pin<Box<dyn Stream<Item = String> + Send>> {
    Box::pin(stream! {
        for step in 1..=steps {
            // simulate "yield" after some async delay
            sleep(Duration::from_millis(10)).await;
            yield format!("Worker {} yielded step {}", id, step);
        }
    })
}

/// Run multiple coroutine workers concurrently and collect results.
pub async fn run_coroutines(num_workers: usize, steps: usize) -> Vec<String> {
    let mut handles = Vec::new();

    for i in 1..=num_workers {
        let mut stream = coroutine_worker(i, steps);
        handles.push(tokio::spawn(async move {
            let mut res = Vec::new();
            while let Some(msg) = stream.next().await {
                res.push(msg);
            }
            res
        }));
    }

    let mut all_results = Vec::new();
    for handle in handles {
        let res = handle.await.unwrap();
        all_results.extend(res);
    }

    all_results
}


