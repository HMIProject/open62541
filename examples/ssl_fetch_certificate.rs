use anyhow::Context as _;
use open62541::{Certificate, PrivateKey};

// These files have been created with `client_ssl.sh`.
const CERTIFICATE_PEM: &[u8] = include_bytes!("client_certificate.pem");
const PRIVATE_KEY_PEM: &[u8] = include_bytes!("client_private_key.pem");

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let certificate = Certificate::from_bytes(CERTIFICATE_PEM);
    let private_key = PrivateKey::from_bytes(PRIVATE_KEY_PEM);

    let certificate =
        open62541::fetch_server_certificate(&certificate, &private_key, "opc.tcp://localhost")
            .context("fetch certificate")?;

    println!("Certificate: {certificate:?}");

    Ok(())
}
