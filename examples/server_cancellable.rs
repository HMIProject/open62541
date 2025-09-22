use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use open62541::Server;

fn main() {
    env_logger::init();

    let (_, runner) = Server::new();

    println!("Starting server");

    let cancel_runner = Arc::new(AtomicBool::new(false));
    let mut is_runner_cancelled = {
        let cancel_runner = Arc::clone(&cancel_runner);
        move || cancel_runner.load(Ordering::Relaxed)
    };

    let runner_task_handle = thread::spawn(move || {
        println!("Server started");
        let result = runner.run_until_cancelled(&mut is_runner_cancelled);
        println!("Server cancelled");
        result
    });

    thread::sleep(Duration::from_secs(15));

    println!("Cancelling server");

    cancel_runner.store(true, Ordering::Relaxed);
    if let Err(err) = runner_task_handle
        .join()
        .expect("runner task should not panic")
    {
        println!("Runner task failed: {err}");
    }

    println!("Exiting");
}
