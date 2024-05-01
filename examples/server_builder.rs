use open62541::ServerBuilder;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building server");

    let (_, runner) = ServerBuilder::default().port(4841).build();

    println!("Running server");

    runner.run()?;

    println!("Exiting");

    Ok(())
}
