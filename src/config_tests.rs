use std::fs;
use std::env;
use tempfile::TempDir;

fn setup_vault(tmp: &TempDir) {
    let vault_dir = tmp.path().join(".vaultkeeper");
    fs::create_dir_all(&vault_dir).unwrap();
    fs::write(
        vault_dir.join("config.toml"),
        "env_file = \".env\"\nlocked_file = \".env.age\"\n",
    ).unwrap();
    fs::write(vault_dir.join("identity.txt"), "AGE-SECRET-KEY-FAKE\n").unwrap();
    fs::write(vault_dir.join("recipient.txt"), "age1fakepublickey\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::VaultConfig;

    #[test]
    fn test_load_config_success() {
        let tmp = tempfile::tempdir().unwrap();
        setup_vault(&tmp);
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let cfg = VaultConfig::load();

        env::set_current_dir(original).unwrap();
        assert!(cfg.is_ok());
        assert_eq!(cfg.unwrap().env_file, ".env");
    }

    #[test]
    fn test_load_config_missing_returns_error() {
        let tmp = tempfile::tempdir().unwrap();
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let cfg = VaultConfig::load();

        env::set_current_dir(original).unwrap();
        assert!(cfg.is_err());
    }

    #[test]
    fn test_is_initialized_true() {
        let tmp = tempfile::tempdir().unwrap();
        setup_vault(&tmp);
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let result = VaultConfig::is_initialized();

        env::set_current_dir(original).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_initialized_false_when_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let result = VaultConfig::is_initialized();

        env::set_current_dir(original).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_identity_and_recipient_trimmed() {
        let tmp = tempfile::tempdir().unwrap();
        setup_vault(&tmp);
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let id = VaultConfig::identity().unwrap();
        let rec = VaultConfig::recipient().unwrap();

        env::set_current_dir(original).unwrap();
        assert_eq!(id, "AGE-SECRET-KEY-FAKE");
        assert_eq!(rec, "age1fakepublickey");
    }
}
