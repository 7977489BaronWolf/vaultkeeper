use crate::config::Config;
use crate::store::env_group::{filter_by_group, missing_keys, parse_groups};
use crate::store::mod_decrypt_store;
use anyhow::{bail, Context, Result};
use std::fs;

/// Handle the `env-group` subcommand.
///
/// Subcommands:
///   list <groups_file>          — list all defined groups
///   show <groups_file> <name>   — show keys in a group and their status
///   export <groups_file> <name> — print KEY=VALUE for all group keys
pub fn handle(cfg: &Config, args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("Usage: vaultkeeper env-group <list|show|export> <groups_file> [group_name]");
    }
    let subcmd = &args[0];
    let groups_file = args.get(1).context("groups_file argument required")?;
    let content = fs::read_to_string(groups_file)
        .with_context(|| format!("Cannot read groups file: {groups_file}"))?;
    let groups = parse_groups(&content);

    match subcmd.as_str() {
        "list" => {
            if groups.is_empty() {
                println!("No groups defined in {groups_file}");
            } else {
                for g in &groups {
                    println!("  {} ({} keys)", g.name, g.keys.len());
                }
            }
        }
        "show" | "export" => {
            let name = args.get(2).context("group name argument required")?;
            let group = groups
                .iter()
                .find(|g| &g.name == name)
                .with_context(|| format!("Group '{name}' not found"))?;

            let secrets = mod_decrypt_store(cfg)?;

            if subcmd == "show" {
                let missing = missing_keys(&secrets, group);
                for key in &group.keys {
                    let status = if missing.contains(key) { "MISSING" } else { "OK" };
                    println!("  [{status}] {key}");
                }
            } else {
                let filtered = filter_by_group(&secrets, group);
                for key in &group.keys {
                    if let Some(val) = filtered.get(key.as_str()) {
                        println!("{}={}", key, val);
                    }
                }
            }
        }
        other => bail!("Unknown env-group subcommand: {other}"),
    }
    Ok(())
}
