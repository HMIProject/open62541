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

    let (certificate, _private_key) = open62541::create_certificate(
        &subject,
        &subject_alt_name,
        &ua::CertificateFormat::DER,
        Some(&params),
    )
    .context("create certificate")?;

    let certificate = certificate.into_x509().context("parse certificate")?;

    println!(
        "Subject common name: {:?}",
        certificate.subject_common_name()
    );
    println!("Key algorithm: {:?}", certificate.key_algorithm());
    println!(
        "Signature algorithm: {:?}",
        certificate.signature_algorithm()
    );
    println!(
        "Validity not before: {:?}",
        certificate.validity_not_before()
    );
    println!("Validity not after: {:?}", certificate.validity_not_after());
    println!(
        "Fingerprint (SHA-1): {:?}",
        certificate
            .sha1_fingerprint()
            .context("SHA-1 fingerprint")?
    );
    println!(
        "Fingerprint (SHA-256): {:?}",
        certificate
            .sha256_fingerprint()
            .context("SHA-256 fingerprint")?
    );
    println!();
    println!(
        "{}",
        certificate.encode_pem().context("encode certificate")?
    );

    Ok(())
}
