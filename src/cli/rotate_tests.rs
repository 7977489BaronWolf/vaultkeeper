#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::cli::rotate::handle_rotate;
    use crate::config::Config;
    use crate::store::{VaultMeta, VaultStore};
    use crate::cli::keygen::generate_key_pair;

    fn make_config(store_path: std::path::PathBuf) -> Config {
        Config {
            store_path,
            default_env_file: std::path::PathBuf::from(".env"),
        }
    }

    #[test]
    fn test_rotate_nonexistent_vault_fails() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("store.json");
        let store = VaultStore::new();
        store.save(&store_path).unwrap();

        let config = make_config(store_path);
        let old_key = dir.path().join("old.key");
        let new_key = dir.path().join("new.key");

        let result = handle_rotate("ghost", &old_key, &new_key, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_rotate_unlocked_vault_fails() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("store.json");
        let enc_path = dir.path().join("myapp.env.age");

        let mut store = VaultStore::new();
        store.add_vault(
            "myapp",
            VaultMeta {
                encrypted_path: enc_path.clone(),
                is_locked: false,
                created_at: "2024-01-01".into(),
            },
        );
        store.save(&store_path).unwrap();

        let config = make_config(store_path);
        let old_key = dir.path().join("old.key");
        let new_key = dir.path().join("new.key");

        let result = handle_rotate("myapp", &old_key, &new_key, &config);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("must be locked"), "unexpected error: {}", msg);
    }

    #[test]
    fn test_rotate_with_valid_keys_succeeds() {
        let dir = tempdir().unwrap();
        let store_path = dir.path().join("store.json");
        let enc_path = dir.path().join("myapp.env.age");
        let plaintext = b"SECRET=hunter2\nAPI_KEY=abc123";

        // Generate two key pairs
        let old_key_path = dir.path().join("old.key");
        let new_key_path = dir.path().join("new.key");
        let old_pub = generate_key_pair(&old_key_path).unwrap();
        let new_pub = generate_key_pair(&new_key_path).unwrap();

        // Encrypt with old key
        crate::crypto::encrypt::encrypt_file(plaintext, &enc_path, &old_key_path).unwrap();

        let mut store = VaultStore::new();
        store.add_vault(
            "myapp",
            VaultMeta {
                encrypted_path: enc_path.clone(),
                is_locked: true,
                created_at: "2024-01-01".into(),
            },
        );
        store.save(&store_path).unwrap();

        let config = make_config(store_path);
        let result = handle_rotate("myapp", &old_key_path, &new_key_path, &config);
        assert!(result.is_ok(), "rotate failed: {:?}", result);

        // Verify the new key can decrypt
        let decrypted =
            crate::crypto::decrypt::decrypt_file(&enc_path, &new_key_path).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
