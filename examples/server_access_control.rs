use anyhow::Context as _;
use open62541::{
    ua, Certificate, DefaultAccessControl, PrivateKey, ServerBuilder, DEFAULT_PORT_NUMBER,
};

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
    .access_control(DefaultAccessControl::new(
        false,
        &[
            (
                &ua::String::new("lorem").expect("create first username"),
                &ua::String::new("lorem123").expect("create first password"),
            ),
            (
                &ua::String::new("ipsum").expect("create second username"),
                &ua::String::new("ipsum123").expect("create second password"),
            ),
        ],
    ))
    .context("set access control")?
    .accept_all()
    .build();

    println!("Running server");

    runner.run_until_interrupt()?;

    println!("Exiting");

    Ok(())
}
