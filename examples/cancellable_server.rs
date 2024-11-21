use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use open62541::ServerBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building server");
    let (_, runner) = ServerBuilder::default().port(4841).build();

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

    println!("Stopping server");
    cancel_runner.store(true, Ordering::Relaxed);
    runner_handle.await.unwrap().unwrap();

    println!("Exiting");

    Ok(())
}
