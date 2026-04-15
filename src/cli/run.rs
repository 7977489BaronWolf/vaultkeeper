use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Command;
use crate::crypto::decrypt::decrypt_env_file;

/// Run a command with decrypted secrets injected as environment variables.
pub fn run_with_secrets(
    vault_path: &str,
    identity_path: &str,
    command: &[String],
) -> Result<()> {
    if command.is_empty() {
        anyhow::bail!("No command provided to run");
    }

    let decrypted = decrypt_env_file(vault_path, identity_path)
        .context("Failed to decrypt vault secrets")?;

    let env_vars = parse_env_contents(&decrypted)
        .context("Failed to parse decrypted env contents")?;

    println!("[vaultkeeper] Injecting {} secret(s) into environment", env_vars.len());

    let program = &command[0];
    let args = &command[1..];

    let status = Command::new(program)
        .args(args)
        .envs(&env_vars)
        .status()
        .with_context(|| format!("Failed to execute command: {}", program))?;

    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }

    Ok(())
}

/// Parse key=value lines from env file contents into a HashMap.
pub fn parse_env_contents_pub(contents: &str) -> Result<HashMap<String, String>> {
    parse_env_contents(contents)
}

fn parse_env_contents(contents: &str) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for (line_number, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            if key.is_empty() {
                anyhow::bail!("Empty key on line {}", line_number + 1);
            }
            // Keys must not contain whitespace, as shells and most tools reject them
            if key.contains(char::is_whitespace) {
                anyhow::bail!(
                    "Key on line {} contains whitespace: {:?}",
                    line_number + 1,
                    key
                );
            }
            map.insert(key.to_string(), value.trim().to_string());
        } else {
            anyhow::bail!("Invalid env line {} (expected KEY=VALUE): {}", line_number + 1, line);
        }
    }
    Ok(map)
}
