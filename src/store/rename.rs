use crate::store::Store;
use anyhow::{bail, Result};

impl Store {
    /// Rename an existing environment, preserving all its key-value pairs.
    pub fn rename_env(&mut self, old_name: &str, new_name: &str) -> Result<()> {
        if !self.has_env(old_name) {
            bail!("Environment '{}' not found", old_name);
        }
        if self.has_env(new_name) {
            bail!("Environment '{}' already exists", new_name);
        }

        // Extract the old entry and reinsert under the new name.
        let entries = self
            .envs
            .remove(old_name)
            .expect("has_env returned true but remove returned None");

        self.envs.insert(new_name.to_string(), entries);

        // Update default_env pointer if it was pointing at the renamed env.
        if self.default_env.as_deref() == Some(old_name) {
            self.default_env = Some(new_name.to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::store::Store;

    #[test]
    fn test_rename_preserves_keys() {
        let mut store = Store::new();
        store.add_env("dev");
        store.set("dev", "DB_URL", "postgres://localhost").unwrap();
        store.set("dev", "PORT", "5432").unwrap();

        store.rename_env("dev", "development").unwrap();

        assert!(!store.has_env("dev"));
        assert!(store.has_env("development"));
        assert_eq!(
            store.get("development", "DB_URL").unwrap(),
            "postgres://localhost"
        );
        assert_eq!(store.get("development", "PORT").unwrap(), "5432");
    }

    #[test]
    fn test_rename_updates_default_env() {
        let mut store = Store::new();
        store.add_env("old");
        store.default_env = Some("old".to_string());

        store.rename_env("old", "new").unwrap();

        assert_eq!(store.default_env.as_deref(), Some("new"));
    }

    #[test]
    fn test_rename_missing_env_errors() {
        let mut store = Store::new();
        let result = store.rename_env("missing", "anything");
        assert!(result.is_err());
    }
}
