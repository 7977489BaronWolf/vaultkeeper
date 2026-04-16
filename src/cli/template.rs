use crate::config::Config;
use crate::store::template::{extract_placeholders, render_template, validate_template};
use std::fs;

pub fn run_render(template_path: &str, env_name: &str, config: &Config) -> anyhow::Result<()> {
    let template = fs::read_to_string(template_path)
        .map_err(|e| anyhow::anyhow!("Failed to read template '{}': {}", template_path, e))?;

    let vars = crate::store::plain::load_plain(env_name, config)?;

    let missing = validate_template(&template, &vars);
    if !missing.is_empty() {
        eprintln!("Warning: missing variables in env '{}': {}", env_name, missing.join(", "));
    }

    let rendered = render_template(&template, &vars);
    print!("{}", rendered);
    Ok(())
}

pub fn run_placeholders(template_path: &str) -> anyhow::Result<()> {
    let template = fs::read_to_string(template_path)
        .map_err(|e| anyhow::anyhow!("Failed to read template '{}': {}", template_path, e))?;

    let keys = extract_placeholders(&template);
    if keys.is_empty() {
        println!("No placeholders found in '{}'.", template_path);
    } else {
        println!("Placeholders in '{}':", template_path);
        for key in &keys {
            println!("  {}", key);
        }
    }
    Ok(())
}
