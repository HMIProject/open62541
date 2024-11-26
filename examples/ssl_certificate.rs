use std::{fs::File, io::Write};

use anyhow::Context as _;
use open62541::ua;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let subject = ua::Array::from_slice(&[
        ua::String::new("C=DE").context("create string")?,
        ua::String::new("O=SampleOrganization").context("create string")?,
        ua::String::new("CN=Open62541Server@localhost").context("create string")?,
    ]);

    let subject_alt_name = ua::Array::from_slice(&[
        ua::String::new("DNS:localhost").context("create string")?,
        ua::String::new("URI:urn:open62541.server.application").context("create string")?,
    ]);

    let params = ua::KeyValueMap::from_slice(&[
        (
            &ua::QualifiedName::ns0("key-size-bits"),
            &ua::Variant::scalar(ua::UInt16::new(4096)),
        ),
        (
            &ua::QualifiedName::ns0("expires-in-days"),
            &ua::Variant::scalar(ua::UInt16::new(30)),
        ),
    ]);

    let certificate = open62541::create_certificate(
        &subject,
        &subject_alt_name,
        &ua::CertificateFormat::DER,
        Some(&params),
    )
    .context("create certificate")?;

    File::create("private_key.der")
        .unwrap()
        .write_all(certificate.private_key.as_bytes().unwrap())
        .unwrap();
    File::create("certificate.der")
        .unwrap()
        .write_all(certificate.certificate.as_bytes().unwrap())
        .unwrap();

    Ok(())
}
