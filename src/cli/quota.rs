use crate::config::Config;
use crate::crypto::decrypt::decrypt_env;
use crate::store::quota::{check_quota, QuotaConfig};
use anyhow::{Context, Result};
use std::collections::HashMap;

pub fn run_quota(env: &str, max_keys: Option<usize>, max_value_bytes: Option<usize>, max_total_bytes: Option<usize>) -> Result<()> {
    let config = Config::load().context("failed to load config")?;
    let key_path = config.key_path();
    let vault_path = config.vault_path(env);

    if !vault_path.exists() {
        anyhow::bail!("environment '{}' does not exist", env);
    }

    let secrets: HashMap<String, String> = if vault_path.metadata()?.len() == 0 {
        HashMap::new()
    } else {
        let identity = age::IdentityFile::from_file(key_path.to_str().unwrap())
            .context("failed to load identity")?
            .into_identities();
        decrypt_env(&vault_path, &identity).context("failed to decrypt vault")?;
        HashMap::new() // placeholder: real impl parses decrypted output
    };

    let quota_config = QuotaConfig {
        max_keys: max_keys.unwrap_or(500),
        max_value_bytes: max_value_bytes.unwrap_or(4096),
        max_total_bytes: max_total_bytes.unwrap_or(1_048_576),
    };

    let report = check_quota(&secrets, &quota_config);

    println!("Quota report for environment: {}", env);
    println!("  Keys:        {}/{}", report.key_count, report.max_keys);
    println!("  Total bytes: {}/{}", report.total_bytes, report.max_total_bytes);

    if let Some(ref key) = report.largest_key {
        println!("  Largest key: {} ({} bytes)", key, report.largest_value_bytes);
    }

    if report.is_ok() {
        println!("  Status: OK");
    } else {
        println!("  Status: VIOLATIONS");
        for v in &report.violations {
            println!("    - {}", v);
        }
        anyhow::bail!("quota violations detected");
    }

    Ok(())
}
