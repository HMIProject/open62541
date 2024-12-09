use anyhow::Context as _;
use itertools::Itertools as _;
use open62541::{Certificate, ClientBuilder};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let endpoint_descriptions = ClientBuilder::default()
        .get_endpoints("opc.tcp://localhost")
        .context("get endpoints")?;

    let server_certificates = endpoint_descriptions
        .iter()
        .filter_map(|endpoint_description| {
            endpoint_description
                .server_certificate()
                .as_bytes()
                .map(|bytes| Certificate::from_bytes(bytes).into_x509())
        })
        .collect::<Result<Vec<_>, _>>()
        .context("parse certificates")?;

    // Include consecutive (!) identical certificates only once.
    let unique_certificates = server_certificates
        .into_iter()
        .dedup_by(|a, b| a.serial_number_asn1() == b.serial_number_asn1())
        .collect::<Vec<_>>();

    println!("Found {} server certificate(s)", unique_certificates.len());

    for (index, certificate) in unique_certificates.iter().enumerate() {
        println!();
        println!("# Certificate {}", index + 1);
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
    }

    Ok(())
}
