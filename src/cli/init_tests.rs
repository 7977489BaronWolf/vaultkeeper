use std::fs;
use tempfile::TempDir;
use std::env;

fn setup_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_creates_vault_dir() {
        let tmp = setup_temp_dir();
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let result = crate::cli::init::run(false);

        env::set_current_dir(original).unwrap();
        assert!(result.is_ok(), "init should succeed: {:?}", result);
        assert!(tmp.path().join(".vaultkeeper").exists());
    }

    #[test]
    fn test_init_creates_identity_and_recipient() {
        let tmp = setup_temp_dir();
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        crate::cli::init::run(false).unwrap();

        env::set_current_dir(original).unwrap();
        assert!(tmp.path().join(".vaultkeeper/identity.txt").exists());
        assert!(tmp.path().join(".vaultkeeper/recipient.txt").exists());
    }

    #[test]
    fn test_init_creates_config() {
        let tmp = setup_temp_dir();
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        crate::cli::init::run(false).unwrap();

        env::set_current_dir(original).unwrap();
        let config = fs::read_to_string(tmp.path().join(".vaultkeeper/config.toml")).unwrap();
        assert!(config.contains("env_file"));
    }

    #[test]
    fn test_init_fails_if_already_initialized_without_force() {
        let tmp = setup_temp_dir();
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        crate::cli::init::run(false).unwrap();
        let second = crate::cli::init::run(false);

        env::set_current_dir(original).unwrap();
        assert!(second.is_err());
    }

    #[test]
    fn test_init_force_reinitializes() {
        let tmp = setup_temp_dir();
        let original = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        crate::cli::init::run(false).unwrap();
        let second = crate::cli::init::run(true);

        env::set_current_dir(original).unwrap();
        assert!(second.is_ok());
    }
}
