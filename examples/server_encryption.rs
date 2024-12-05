use anyhow::Context as _;
use open62541::{Certificate, PrivateKey, ServerBuilder, DEFAULT_PORT_NUMBER};

// These files have been created with `server_ssl.sh`.
const CERTIFICATE_PEM: &[u8] = include_bytes!("server_certificate.pem");
const PRIVATE_KEY_PEM: &[u8] = include_bytes!("server_private_key.pem");

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building server");

    let certificate = Certificate::from_bytes(CERTIFICATE_PEM);
    let private_key = PrivateKey::from_bytes(PRIVATE_KEY_PEM);

    let (_, runner) = ServerBuilder::default_with_security_policies(
        DEFAULT_PORT_NUMBER,
        &certificate,
        &private_key,
    )
    .context("get server builder")?
    .accept_all()
    .build();

    println!("Running server");

    runner.run()?;

    println!("Exiting");

    Ok(())
}
