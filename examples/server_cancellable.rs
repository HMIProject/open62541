use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use open62541::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (_, runner) = Server::new();

    println!("Starting server");

    let cancel_runner = Arc::new(AtomicBool::new(false));
    let mut is_runner_cancelled = {
        let cancel_runner = Arc::clone(&cancel_runner);
        move || cancel_runner.load(Ordering::Relaxed)
    };

    let runner_handle = tokio::task::spawn_blocking(move || {
        println!("Server started");
        let result = runner.run_until_cancelled(&mut is_runner_cancelled);
        println!("Server cancelled");
        result
    });

    tokio::time::sleep(Duration::from_secs(15)).await;

    println!("Cancelling server");

    cancel_runner.store(true, Ordering::Relaxed);
    runner_handle.await.unwrap().unwrap();

    println!("Exiting");

    Ok(())
}
