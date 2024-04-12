use open62541::ServerBuilder;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building server");

    let server = ServerBuilder::default().port(4841).build();

    println!("Running server");

    server.run()?;

    println!("Exiting");

    Ok(())
}
