use age::x25519;
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Generates a new age X25519 key pair.
/// Writes the private key to `output_path` and prints the public key to stdout.
pub fn run(output_path: Option<&Path>) -> Result<()> {
    let default_path = Path::new("vaultkeeper.key");
    let key_path = output_path.unwrap_or(default_path);

    if key_path.exists() {
        anyhow::bail!(
            "Key file already exists at '{}'. Remove it first or specify a different path.",
            key_path.display()
        );
    }

    let secret_key = x25519::Identity::generate();
    let public_key = secret_key.to_public();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(key_path)
        .with_context(|| format!("Failed to create key file at '{}'", key_path.display()))?;

    writeln!(file, "# vaultkeeper age private key")?;
    writeln!(file, "# Public key: {}", public_key)?;
    writeln!(file, "{}", secret_key.to_string())?;

    println!("Public key : {}", public_key);
    println!("Private key: {}", key_path.display());
    println!("\nKeep the private key secret. Add it to .gitignore.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn generates_key_file() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("test.key");
        run(Some(&key_path)).expect("keygen should succeed");
        let contents = fs::read_to_string(&key_path).unwrap();
        assert!(contents.contains("AGE-SECRET-KEY-"));
        assert!(contents.contains("vaultkeeper age private key"));
    }

    #[test]
    fn fails_if_key_file_exists() {
        let dir = tempdir().unwrap();
        let key_path = dir.path().join("existing.key");
        fs::write(&key_path, "existing").unwrap();
        let result = run(Some(&key_path));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
}
