use std::sync::{Arc, Mutex};

use anyhow::Context as _;
use open62541::{ua, DefaultAccessControlWithLoginCallback, ServerBuilder, DEFAULT_PORT_NUMBER};

struct Credentials {
    user_name: String,
    password: String,
}

impl Credentials {
    fn new(user_name: &str, password: &str) -> Self {
        Self {
            user_name: user_name.into(),
            password: password.into(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building server");

    // These files have been created with `server_ssl.sh`.
    let certificate_pem = include_str!("server_certificate.pem");
    let private_key_pem = include_str!("server_private_key.pem");

    let certificate = pem::parse(certificate_pem).context("parse PEM certificate")?;
    let private_key = pem::parse(private_key_pem).context("parse PEM private key")?;

    let credentials = Arc::new(Mutex::new(vec![
        Credentials::new("lorem", "lorem123"),
        Credentials::new("ipsum", "ipsum123"),
    ]));

    let login_callback =
        move |user_name: &ua::String, password: &ua::ByteString| -> ua::StatusCode {
            let Some(user_name) = user_name.as_str() else {
                return ua::StatusCode::BADINTERNALERROR;
            };
            let Some(password) = password.as_bytes() else {
                return ua::StatusCode::BADINTERNALERROR;
            };

            let Ok(credentials) = credentials.lock() else {
                return ua::StatusCode::BADINTERNALERROR;
            };

            if credentials.iter().any(|credentials| {
                credentials.user_name == user_name && credentials.password.as_bytes() == password
            }) {
                ua::StatusCode::GOOD
            } else {
                ua::StatusCode::BADUSERACCESSDENIED
            }
        };

    let (_, runner) = ServerBuilder::default_with_security_policies(
        DEFAULT_PORT_NUMBER,
        certificate.contents(),
        private_key.contents(),
    )
    .context("get server builder")?
    .access_control(DefaultAccessControlWithLoginCallback::new(
        false,
        login_callback,
    ))
    .context("set access control")?
    .accept_all()
    .build();

    println!("Running server");

    runner.run()?;

    println!("Exiting");

    Ok(())
}
