use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Context as _;
use open62541::{
    ua, Certificate, DefaultAccessControlWithLoginCallback, PrivateKey, ServerBuilder, DEFAULT_PORT_NUMBER
};

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

    let credentials = Arc::new(Mutex::new(vec![
        Credentials::new("lorem", "lorem123"),
        Credentials::new("ipsum", "ipsum123"),
    ]));

    // As an example for concurrent access in the closure below, we clear the credentials after some
    // time. Future attempts at logging in will then be rejected.
    //
    thread::spawn({
        let credentials = Arc::downgrade(&credentials);

        move || {
            thread::sleep(Duration::from_secs(15));

            let Some(credentials) = credentials.upgrade() else {
                return;
            };
            let Ok(mut credentials) = credentials.lock() else {
                return;
            };

            println!("Clearing credentials");

            credentials.clear();
        }
    });

    // For each login, we look into the current set of `credentials`.
    //
    let login_callback = {
        let credentials = Arc::clone(&credentials);

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

            println!("Checking credentials for {user_name:?}");

            if credentials.iter().any(|credentials| {
                credentials.user_name == user_name && credentials.password.as_bytes() == password
            }) {
                ua::StatusCode::GOOD
            } else {
                ua::StatusCode::BADUSERACCESSDENIED
            }
        }
    };

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
