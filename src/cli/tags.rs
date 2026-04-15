use crate::config::Config;
use crate::store::tags::{extract_tags, filter_by_tag};
use std::fs;

/// Handles the `vaultkeeper tags list <env>` subcommand.
/// Prints all unique tags found in the given environment file.
pub fn handle_list_tags(env: &str, config: &Config) -> anyhow::Result<()> {
    let path = config.env_file_path(env);
    let content = fs::read_to_string(&path)
        .map_err(|_| anyhow::anyhow!("Environment '{}' not found at {:?}", env, path))?;

    let tag_map = extract_tags(&content);
    let mut all_tags: Vec<String> = tag_map
        .values()
        .flatten()
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    all_tags.sort();

    if all_tags.is_empty() {
        println!("No tags found in environment '{}'.", env);
    } else {
        println!("Tags in '{}':", env);
        for tag in &all_tags {
            println!("  - {}", tag);
        }
    }

    Ok(())
}

/// Handles the `vaultkeeper tags filter <env> <tag>` subcommand.
/// Prints all key=value pairs in the environment that have the given tag.
pub fn handle_filter_by_tag(env: &str, tag: &str, config: &Config) -> anyhow::Result<()> {
    let path = config.env_file_path(env);
    let content = fs::read_to_string(&path)
        .map_err(|_| anyhow::anyhow!("Environment '{}' not found at {:?}", env, path))?;

    let matches = filter_by_tag(&content, tag);

    if matches.is_empty() {
        println!("No variables tagged '{}' in environment '{}'.", tag, env);
    } else {
        println!("Variables tagged '{}' in '{}':", tag, env);
        for line in matches {
            println!("  {}", line.trim());
        }
    }

    Ok(())
}
