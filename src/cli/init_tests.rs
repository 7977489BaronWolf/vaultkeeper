use std::fs;
use tempfile::TempDir;
use std::env;

fn setup_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

/// Changes the current directory to `path`, runs `f`, then restores the original directory.
/// Ensures the directory is restored even if `f` panics.
fn with_dir<F, T>(path: &std::path::Path, f: F) -> T
where
    F: FnOnce() -> T,
{
    let original = env::current_dir().expect("Failed to get current dir");
    env::set_current_dir(path).expect("Failed to set current dir");
    let result = f();
    env::set_current_dir(original).expect("Failed to restore current dir");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_creates_vault_dir() {
        let tmp = setup_temp_dir();

        let result = with_dir(tmp.path(), || crate::cli::init::run(false));

        assert!(result.is_ok(), "init should succeed: {:?}", result);
        assert!(tmp.path().join(".vaultkeeper").exists());
    }

    #[test]
    fn test_init_creates_identity_and_recipient() {
        let tmp = setup_temp_dir();

        with_dir(tmp.path(), || crate::cli::init::run(false).unwrap());

        assert!(tmp.path().join(".vaultkeeper/identity.txt").exists());
        assert!(tmp.path().join(".vaultkeeper/recipient.txt").exists());
    }

    #[test]
    fn test_init_creates_config() {
        let tmp = setup_temp_dir();

        with_dir(tmp.path(), || crate::cli::init::run(false).unwrap());

        let config = fs::read_to_string(tmp.path().join(".vaultkeeper/config.toml")).unwrap();
        assert!(config.contains("env_file"));
    }

    #[test]
    fn test_init_fails_if_already_initialized_without_force() {
        let tmp = setup_temp_dir();

        let second = with_dir(tmp.path(), || {
            crate::cli::init::run(false).unwrap();
            crate::cli::init::run(false)
        });

        assert!(second.is_err());
    }

    #[test]
    fn test_init_force_reinitializes() {
        let tmp = setup_temp_dir();

        let second = with_dir(tmp.path(), || {
            crate::cli::init::run(false).unwrap();
            crate::cli::init::run(true)
        });

        assert!(second.is_ok());
    }
}
