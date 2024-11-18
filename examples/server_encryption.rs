use anyhow::Context as _;
use open62541::{ServerBuilder, DEFAULT_PORT_NUMBER};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building server");

    // These files have been created with OpenSSL:
    //
    // ```
    // openssl req -x509 -newkey rsa:4096 \
    //     -keyout server_test.key -out server_test.crt -sha256 -days 3650 -nodes \
    //     -subj "/C=DE/O=open62541/CN=open62541@localhost" \
    //     -addext "subjectAltName=DNS:localhost,URI:urn:open62541.server.application"
    // ```
    let certificate_pem = include_str!("server_test.crt");
    let private_key_pem = include_str!("server_test.key");

    let certificate = pem::parse(certificate_pem).context("parse PEM certificate")?;
    let private_key = pem::parse(private_key_pem).context("parse PEM private key")?;

    let (_, runner) = ServerBuilder::default_with_security_policies(
        DEFAULT_PORT_NUMBER,
        certificate.contents(),
        private_key.contents(),
    )
    .context("get server builder")?
    .build();

    println!("Running server");

    runner.run()?;

    println!("Exiting");

    Ok(())
}
