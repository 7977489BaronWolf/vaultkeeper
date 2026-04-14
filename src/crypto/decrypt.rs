use age::{
    decrypt,
    x25519::Identity,
};
use anyhow::{Context, Result};
use std::{
    fs,
    io::Read,
    path::Path,
    str::FromStr,
};

/// Decrypts an age-encrypted env file using the given private key file path.
/// Writes the plaintext output to `output_path`.
pub fn decrypt_env_file(
    input_path: &Path,
    output_path: &Path,
    identity_path: &Path,
) -> Result<()> {
    let key_contents = fs::read_to_string(identity_path)
        .with_context(|| format!("Failed to read identity file: {}", identity_path.display()))?;

    let identity = key_contents
        .lines()
        .find(|line| line.starts_with("AGE-SECRET-KEY-"))
        .ok_or_else(|| anyhow::anyhow!("No secret key found in: {}", identity_path.display()))?;

    let identity = Identity::from_str(identity)
        .map_err(|e| anyhow::anyhow!("Invalid age identity: {}", e))?;

    let ciphertext = fs::read(input_path)
        .with_context(|| format!("Failed to read encrypted file: {}", input_path.display()))?;

    let decryptor = match decrypt::Decryptor::new(&ciphertext[..]).context("Failed to parse encrypted file")? {
        decrypt::Decryptor::Recipients(d) => d,
        _ => anyhow::bail!("Unexpected encryption type (passphrase-based not supported)"),
    };

    let mut reader = decryptor
        .decrypt(std::iter::once(&identity as &dyn age::Identity))
        .context("Decryption failed — wrong key?")?;

    let mut plaintext = vec![];
    reader.read_to_end(&mut plaintext).context("Failed to read decrypted data")?;

    fs::write(output_path, &plaintext)
        .with_context(|| format!("Failed to write decrypted file: {}", output_path.display()))?;

    println!(
        "Decrypted '{}' -> '{}'",
        input_path.display(),
        output_path.display()
    );

    Ok(())
}
