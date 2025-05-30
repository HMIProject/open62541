use open62541::ServerBuilder;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (server, runner) = ServerBuilder::default()
        .server_urls(&["opc.tcp://127.0.0.1:0"])
        .build();

    let handle = std::thread::spawn(|| runner.run_until_cancelled(&mut || true).unwrap());

    match server.discovery_urls() {
        None => {
            println!("No discovery URLs");
        }
        Some(urls) => {
            println!("Discovery URLs: {:?}", urls);

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
        }
    }

    handle.join().unwrap();

    Ok(())
}
