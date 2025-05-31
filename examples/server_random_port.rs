use std::thread;

use open62541::ServerBuilder;

fn main() {
    env_logger::init();

    let (mut server, runner) = ServerBuilder::default()
        .server_urls(&["opc.tcp://127.0.0.1:0"])
        .build();

    let handle = thread::spawn(|| runner.run_until_cancelled(&mut || true).unwrap());

    if let Some(urls) = server.discovery_urls() {
        println!("Discovery URLs: {urls:?}");

        if let Some(tail) = urls
            .as_slice()
            .first()
            .and_then(|url| url.as_str())
            .and_then(|url| url.strip_prefix("opc.tcp://127.0.0.1:"))
        {
            if let Ok(port) = tail.parse::<u16>() {
                println!("Server running on port {port}");
            } else {
                println!("Unable to find port");
            }
        }
    } else {
        println!("No discovery URLs");
    }

    handle.join().unwrap();
}
