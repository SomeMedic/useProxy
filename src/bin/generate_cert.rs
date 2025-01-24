use rcgen::{Certificate, CertificateParams, DnType, SanType};
use std::fs;
use std::net::IpAddr;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut params = CertificateParams::default();
    params.distinguished_name.push(DnType::CommonName, "localhost");
    params.subject_alt_names = vec![
        SanType::DnsName("localhost".to_string()),
        SanType::IpAddress(IpAddr::from_str("127.0.0.1")?),
    ];

    let cert = Certificate::from_params(params)?;
    
    // Сохраняем сертификат
    fs::write("cert.pem", cert.serialize_pem()?)?;
    
    // Сохраняем приватный ключ
    fs::write("key.pem", cert.serialize_private_key_pem())?;

    println!("Сертификаты успешно сгенерированы:");
    println!("cert.pem - сертификат");
    println!("key.pem - приватный ключ");

    Ok(())
} 