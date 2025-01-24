use std::path::Path;
use std::process::Command;

pub fn generate_self_signed_cert(
    _cert_path: &Path,
    _key_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Запускаем отдельный процесс для генерации сертификатов
    let status = Command::new("cargo")
        .args(&["run", "--bin", "generate_cert"])
        .status()?;

    if !status.success() {
        return Err("Failed to generate certificates".into());
    }

    Ok(())
} 