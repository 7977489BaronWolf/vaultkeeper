use crate::store::token::{Token, TokenStore};
use anyhow::{anyhow, Result};
use uuid::Uuid;

pub fn cmd_token_create(
    store: &mut TokenStore,
    label: &str,
    scopes: Vec<String>,
    ttl_secs: Option<u64>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let expires_at = ttl_secs.map(|secs| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            + secs
    });
    let token = Token::new(&id, label, scopes, expires_at);
    store.add(token);
    println!("Token created: {}", id);
    Ok(id)
}

pub fn cmd_token_revoke(store: &mut TokenStore, id: &str) -> Result<()> {
    store
        .remove(id)
        .ok_or_else(|| anyhow!("Token '{}' not found", id))?;
    println!("Token '{}' revoked.", id);
    Ok(())
}

pub fn cmd_token_list(store: &TokenStore) -> Result<()> {
    let tokens = store.list_valid();
    if tokens.is_empty() {
        println!("No active tokens.");
        return Ok(());
    }
    println!("{:<36}  {:<20}  {}", "ID", "LABEL", "SCOPES");
    for t in tokens {
        println!("{:<36}  {:<20}  {}", t.id, t.label, t.scopes.join(","));
    }
    Ok(())
}

pub fn cmd_token_purge(store: &mut TokenStore) -> Result<()> {
    let n = store.purge_expired();
    println!("Purged {} expired token(s).", n);
    Ok(())
}

pub fn cmd_token_inspect(store: &TokenStore, id: &str) -> Result<()> {
    let token = store.get(id).ok_or_else(|| anyhow!("Token '{}' not found", id))?;
    println!("ID:         {}", token.id);
    println!("Label:      {}", token.label);
    println!("Scopes:     {}", token.scopes.join(", "));
    println!("Created:    {}", token.created_at);
    println!("Expires:    {}", token.expires_at.map(|e| e.to_string()).unwrap_or_else(|| "never".into()));
    println!("Expired:    {}", token.is_expired());
    Ok(())
}
