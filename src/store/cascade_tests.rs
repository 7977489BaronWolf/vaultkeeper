#[cfg(test)]
mod tests {
    use super::super::cascade::CascadeChain;
    use std::collections::HashMap;

    fn make_stores() -> HashMap<String, HashMap<String, String>> {
        let mut stores = HashMap::new();

        let mut base = HashMap::new();
        base.insert("DB_HOST".to_string(), "localhost".to_string());
        base.insert("DB_PORT".to_string(), "5432".to_string());
        base.insert("LOG_LEVEL".to_string(), "info".to_string());
        stores.insert("base".to_string(), base);

        let mut staging = HashMap::new();
        staging.insert("DB_HOST".to_string(), "staging.db".to_string());
        staging.insert("API_KEY".to_string(), "stg-key-123".to_string());
        stores.insert("staging".to_string(), staging);

        let mut prod = HashMap::new();
        prod.insert("DB_HOST".to_string(), "prod.db".to_string());
        prod.insert("API_KEY".to_string(), "prod-key-456".to_string());
        prod.insert("LOG_LEVEL".to_string(), "warn".to_string());
        stores.insert("prod".to_string(), prod);

        stores
    }

    #[test]
    fn test_resolve_highest_priority_wins() {
        let stores = make_stores();
        let chain = CascadeChain::new(vec!["prod".to_string(), "staging".to_string(), "base".to_string()]);
        let (layer, value) = chain.resolve("DB_HOST", &stores).unwrap();
        assert_eq!(layer, "prod");
        assert_eq!(value, "prod.db");
    }

    #[test]
    fn test_resolve_falls_through_to_base() {
        let stores = make_stores();
        let chain = CascadeChain::new(vec!["prod".to_string(), "base".to_string()]);
        let (layer, value) = chain.resolve("DB_PORT", &stores).unwrap();
        assert_eq!(layer, "base");
        assert_eq!(value, "5432");
    }

    #[test]
    fn test_resolve_missing_key_returns_none() {
        let stores = make_stores();
        let chain = CascadeChain::new(vec!["prod".to_string(), "base".to_string()]);
        assert!(chain.resolve("NONEXISTENT", &stores).is_none());
    }

    #[test]
    fn test_flatten_merges_all_layers() {
        let stores = make_stores();
        let chain = CascadeChain::new(vec!["prod".to_string(), "base".to_string()]);
        let flat = chain.flatten(&stores);
        assert_eq!(flat.get("DB_HOST").unwrap(), "prod.db");
        assert_eq!(flat.get("DB_PORT").unwrap(), "5432");
        assert_eq!(flat.get("LOG_LEVEL").unwrap(), "warn");
    }

    #[test]
    fn test_all_keys_sorted_and_deduped() {
        let stores = make_stores();
        let chain = CascadeChain::new(vec!["prod".to_string(), "staging".to_string(), "base".to_string()]);
        let keys = chain.all_keys(&stores);
        assert_eq!(keys, vec!["API_KEY", "DB_HOST", "DB_PORT", "LOG_LEVEL"]);
    }

    #[test]
    fn test_empty_chain_returns_empty() {
        let stores = make_stores();
        let chain = CascadeChain::new(vec![]);
        assert!(chain.resolve("DB_HOST", &stores).is_none());
        assert!(chain.flatten(&stores).is_empty());
    }
}
