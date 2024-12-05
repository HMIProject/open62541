use anyhow::Context as _;
use open62541::{Certificate, PrivateKey, ServerBuilder, DEFAULT_PORT_NUMBER};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building server");

    // These files have been created with `server_ssl.sh`.
    let certificate_pem = include_str!("server_certificate.pem");
    let private_key_pem = include_str!("server_private_key.pem");

    let certificate = pem::parse(certificate_pem).context("parse PEM certificate")?;
    let private_key = pem::parse(private_key_pem).context("parse PEM private key")?;

    let certificate = Certificate::from_bytes(certificate.contents());
    let private_key = PrivateKey::from_bytes(private_key.contents());

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
