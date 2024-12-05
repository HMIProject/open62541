use anyhow::Context as _;
use open62541::{Certificate, ClientBuilder, PrivateKey};

// These files have been created with `client_ssl.sh`.
const CERTIFICATE_PEM: &[u8] = include_bytes!("client_certificate.pem");
const PRIVATE_KEY_PEM: &[u8] = include_bytes!("client_private_key.pem");

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building client");

    let certificate = Certificate::from_bytes(CERTIFICATE_PEM);
    let private_key = PrivateKey::from_bytes(PRIVATE_KEY_PEM);

    let client = ClientBuilder::default_encryption(&certificate, &private_key)
        .context("get client builder")?
        .accept_all()
        .connect("opc.tcp://localhost")
        .context("connect")?;

    println!("Connected successfully");

    println!("Disconnecting client");

    client.disconnect();

    println!("Exiting");

    Ok(())
}
