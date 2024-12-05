use anyhow::Context as _;
use open62541::{Certificate, ClientBuilder, PrivateKey};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building client");

    // These files have been created with `client_ssl.sh`.
    let certificate_pem = include_str!("client_certificate.pem");
    let private_key_pem = include_str!("client_private_key.pem");

    let certificate = pem::parse(certificate_pem).context("parse PEM certificate")?;
    let private_key = pem::parse(private_key_pem).context("parse PEM private key")?;

    let certificate = Certificate::from_bytes(certificate.contents());
    let private_key = PrivateKey::from_bytes(private_key.contents());

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
