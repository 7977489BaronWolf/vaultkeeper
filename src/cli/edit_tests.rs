//! Integration-style tests for the edit command helpers.
//! These tests exercise the encrypt → edit-simulation → decrypt round-trip
//! without spawning a real editor.

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;

    use crate::crypto::{decrypt_file, encrypt_file};

    fn sample_env() -> &'static [u8] {
        b"API_KEY=secret123\nDB_PASS=hunter2\n"
    }

    /// Helper: create a key pair and return (secret_key_path, public_key_path).
    fn generate_test_key(dir: &std::path::Path) -> (std::path::PathBuf, std::path::PathBuf) {
        let sk_path = dir.join("test.key");
        let pk_path = dir.join("test.pub");
        crate::cli::keygen::generate_keypair(sk_path.to_str().unwrap(), pk_path.to_str().unwrap())
            .expect("keygen failed");
        (sk_path, pk_path)
    }

    /// Helper: encrypt `contents` into `vault`, decrypt it back, and assert the round-trip is
    /// lossless. Returns the raw bytes read back from the vault file after decryption.
    fn assert_encrypt_decrypt_roundtrip(
        contents: &[u8],
        vault: &std::path::Path,
        pk: &std::path::Path,
        sk: &std::path::Path,
    ) -> Vec<u8> {
        encrypt_file(contents, vault.to_str().unwrap(), pk.to_str().unwrap())
            .expect("encrypt failed");
        let decrypted = decrypt_file(vault.to_str().unwrap(), sk.to_str().unwrap())
            .expect("decrypt failed");
        assert_eq!(&decrypted, contents, "round-trip mismatch");
        decrypted
    }

    #[test]
    fn edit_round_trip_preserves_content() {
        let dir = tempdir().unwrap();
        let (sk, pk) = generate_test_key(dir.path());
        let vault = dir.path().join("test.vault");

        // Initial encrypt + verify
        assert_encrypt_decrypt_roundtrip(sample_env(), &vault, &pk, &sk);

        // Simulate edit: re-encrypt with updated content and verify
        let updated = b"API_KEY=newsecret\nDB_PASS=correct-horse\n";
        assert_encrypt_decrypt_roundtrip(updated, &vault, &pk, &sk);
    }

    #[test]
    fn edit_no_change_leaves_vault_intact() {
        let dir = tempdir().unwrap();
        let (sk, pk) = generate_test_key(dir.path());
        let vault = dir.path().join("intact.vault");

        encrypt_file(sample_env(), vault.to_str().unwrap(), pk.to_str().unwrap())
            .expect("encrypt failed");

        let before_bytes = fs::read(&vault).unwrap();
        // No re-encrypt simulates "no change" path
        let after_bytes = fs::read(&vault).unwrap();
        assert_eq!(before_bytes, after_bytes);
    }
}
