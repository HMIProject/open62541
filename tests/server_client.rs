use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use open62541::{ua, ClientBuilder, ServerBuilder};
use open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME;

// This mirrors test `create_and_destroy_client()` in `open62541-sys`.
#[test]
fn create_and_destroy_client() {
    // This does not actually connect to anything.
    let _client = ua::Client::default();
}

// This mirrors test `open_server_and_connect()` in `open62541-sys`.
#[tokio::test]
async fn open_server_and_connect() {
    // Initialize new server listening on random port.
    let (server, runner) = ServerBuilder::default()
        .server_urls(&["opc.tcp://127.0.0.1:0"])
        .build();

    // Run server in background thread, iterating event loop.
    let running = Arc::new(AtomicBool::new(true));
    let background_thread = thread::spawn({
        let running = Arc::clone(&running);
        move || {
            runner
                .run_until_cancelled(|| !running.load(Ordering::Relaxed))
                .expect("run server");
        }
    });

    // In main thread, wait for discovery URL (after start-up).
    let discovery_urls = server.discovery_urls().expect("get discovery URLs");
    let discovery_url = discovery_urls
        .as_slice()
        .first()
        .expect("get discovery URL")
        .as_str()
        .expect("valid discovery URL");

    // Connect to given URL.
    let client = ClientBuilder::default()
        .connect(discovery_url)
        .expect("connect client")
        .into_async();

    let value = client
        .read_value(&ua::NodeId::ns0(
            UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME,
        ))
        .await
        .expect("read value");

    let value: ua::String = value
        .into_value()
        .expect("has value")
        .into_scalar()
        .expect("has string value");

    assert!(value
        .as_str()
        .expect("is valid string")
        .contains("open62541 OPC UA Server"));

    // Clean up client.
    client.disconnect().await;

    // Shut down server.
    running.store(false, Ordering::Relaxed);
    background_thread.join().unwrap();
}
