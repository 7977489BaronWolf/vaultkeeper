use age::{
    encrypt,
    x25519::Recipient,
};
use anyhow::{Context, Result};
use std::{
    fs,
    io::Write,
    path::Path,
    str::FromStr,
};

use super::resolve_recipient;

/// Encrypts a plaintext env file using the given age public key (or path to pubkey file).
/// Writes the encrypted output to `output_path`.
pub fn encrypt_env_file(
    input_path: &Path,
    output_path: &Path,
    recipient_str: &str,
) -> Result<()> {
    let pubkey_str = resolve_recipient(recipient_str)
        .context("Failed to resolve recipient public key")?;

    let recipient = Recipient::from_str(&pubkey_str)
        .map_err(|e| anyhow::anyhow!("Invalid age public key: {}", e))?;

    let plaintext = fs::read(input_path)
        .with_context(|| format!("Failed to read input file: {}", input_path.display()))?;

    let encryptor = encrypt::Encryptor::with_recipients(vec![Box::new(recipient)])
        .expect("Failed to create encryptor");

    let mut encrypted = vec![];
    let mut writer = encryptor
        .wrap_output(&mut encrypted)
        .context("Failed to create encryption writer")?;

    writer.write_all(&plaintext).context("Failed to write plaintext to encryptor")?;
    writer.finish().context("Failed to finalize encryption")?;

    fs::write(output_path, &encrypted)
        .with_context(|| format!("Failed to write encrypted file: {}", output_path.display()))?;

    println!(
        "Encrypted '{}' -> '{}'",
        input_path.display(),
        output_path.display()
    );

    Ok(())
}
