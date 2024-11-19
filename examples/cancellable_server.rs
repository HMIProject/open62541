use open62541::ServerBuilder;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let task_tracker = TaskTracker::new();
    let task_tracker_clone = TaskTracker::new();
    let cancellation_token = CancellationToken::new();
    let cancellation_token_clone = cancellation_token.clone();

    println!("Building server");
    tokio::time::sleep(Duration::from_millis(200)).await;
    let (_, runner) = ServerBuilder::default().port(4841).build();
    let _jh = task_tracker.spawn(async move {
        runner
            .run_with_cancellation_token(task_tracker_clone, cancellation_token_clone)
            .await
    });
    println!("Server started");
    tokio::time::sleep(Duration::from_millis(200)).await;

    println!("Stopping server");
    cancellation_token.cancel();
    tokio::time::sleep(Duration::from_millis(200)).await;
    cancellation_token.cancelled().await;

    task_tracker.close();
    task_tracker.wait().await;
    println!("Exiting");

    Ok(())
}
