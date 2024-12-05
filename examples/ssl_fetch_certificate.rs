use anyhow::Context as _;
use open62541::{ua, ClientBuilder, CustomCertificateVerification};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Building client");

    // These files have been created with `client_ssl.sh`.
    let certificate_pem = include_str!("client_certificate.pem");
    let private_key_pem = include_str!("client_private_key.pem");

    let certificate = pem::parse(certificate_pem).context("parse PEM certificate")?;
    let private_key = pem::parse(private_key_pem).context("parse PEM private key")?;

    let client = ClientBuilder::default_encryption(certificate.contents(), private_key.contents())
        .context("get client builder")?
        .certificate_verification(ua::CertificateVerification::custom(FetchCertificate))
        .connect("opc.tcp://localhost")
        .context("connect")?;

    println!("Connected successfully");

    println!("Disconnecting client");

    client.disconnect();

    println!("Exiting");

    Ok(())
}

struct FetchCertificate;

impl CustomCertificateVerification for FetchCertificate {
    fn verify_certificate(&self, certificate: &ua::ByteString) -> ua::StatusCode {
        println!("Certificate: {certificate:?}");
        ua::StatusCode::GOOD
    }

    fn verify_application_uri(
        &self,
        _certificate: &ua::ByteString,
        application_uri: &ua::String,
    ) -> ua::StatusCode {
        println!("Application URI: {application_uri:?}");
        ua::StatusCode::GOOD
    }
}
