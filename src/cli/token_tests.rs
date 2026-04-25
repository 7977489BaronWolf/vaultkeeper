#[cfg(test)]
mod tests {
    use crate::store::token::TokenStore;
    use crate::cli::token::*;

    fn fresh_store() -> TokenStore {
        TokenStore::default()
    }

    #[test]
    fn test_create_token_returns_id() {
        let mut store = fresh_store();
        let id = cmd_token_create(&mut store, "ci-bot", vec!["read".into()], None).unwrap();
        assert!(!id.is_empty());
        assert!(store.get(&id).is_some());
    }

    #[test]
    fn test_create_token_with_ttl() {
        let mut store = fresh_store();
        let id = cmd_token_create(&mut store, "short-lived", vec!["write".into()], Some(3600)).unwrap();
        let token = store.get(&id).unwrap();
        assert!(token.expires_at.is_some());
    }

    #[test]
    fn test_revoke_existing_token() {
        let mut store = fresh_store();
        let id = cmd_token_create(&mut store, "to-revoke", vec!["read".into()], None).unwrap();
        assert!(cmd_token_revoke(&mut store, &id).is_ok());
        assert!(store.get(&id).is_none());
    }

    #[test]
    fn test_revoke_missing_token_errors() {
        let mut store = fresh_store();
        let result = cmd_token_revoke(&mut store, "nonexistent-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_empty_store() {
        let store = fresh_store();
        assert!(cmd_token_list(&store).is_ok());
    }

    #[test]
    fn test_list_with_tokens() {
        let mut store = fresh_store();
        cmd_token_create(&mut store, "bot-a", vec!["read".into()], None).unwrap();
        cmd_token_create(&mut store, "bot-b", vec!["write".into()], None).unwrap();
        assert!(cmd_token_list(&store).is_ok());
    }

    #[test]
    fn test_purge_removes_expired() {
        let mut store = fresh_store();
        cmd_token_create(&mut store, "live", vec!["read".into()], None).unwrap();
        // Manually insert an expired token
        use crate::store::token::Token;
        store.add(Token::new("expired-id", "expired", vec!["read".into()], Some(1)));
        assert!(cmd_token_purge(&mut store).is_ok());
        assert!(store.get("expired-id").is_none());
    }

    #[test]
    fn test_inspect_existing() {
        let mut store = fresh_store();
        let id = cmd_token_create(&mut store, "inspect-me", vec!["*".into()], None).unwrap();
        assert!(cmd_token_inspect(&store, &id).is_ok());
    }

    #[test]
    fn test_inspect_missing_errors() {
        let store = fresh_store();
        assert!(cmd_token_inspect(&store, "ghost").is_err());
    }
}
