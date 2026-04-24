use crate::store::schema::{Schema, SchemaField, SchemaType};
use anyhow::{Context, Result};
use std::path::PathBuf;

fn schema_path(env: &str) -> PathBuf {
    PathBuf::from(format!(".vaultkeeper/{}.schema.json", env))
}

fn load_schema(env: &str) -> Result<Schema> {
    let path = schema_path(env);
    if !path.exists() {
        return Ok(Schema::new());
    }
    let data = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read schema for '{}'", env))?;
    serde_json::from_str(&data).context("Failed to parse schema")
}

fn save_schema(env: &str, schema: &Schema) -> Result<()> {
    let path = schema_path(env);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(schema).context("Failed to serialize schema")?;
    std::fs::write(&path, data).context("Failed to write schema")
}

pub fn handle_schema_add(env: &str, key: &str, field_type: &str, required: bool, description: Option<String>) -> Result<()> {
    let ftype = parse_type(field_type)?;
    let mut schema = load_schema(env)?;
    schema.add_field(key.to_string(), SchemaField {
        field_type: ftype,
        required,
        description,
        pattern: None,
    });
    save_schema(env, &schema)?;
    println!("Schema field '{}' added to env '{}'.", key, env);
    Ok(())
}

pub fn handle_schema_remove(env: &str, key: &str) -> Result<()> {
    let mut schema = load_schema(env)?;
    if schema.remove_field(key) {
        save_schema(env, &schema)?;
        println!("Schema field '{}' removed from env '{}'.", key, env);
    } else {
        println!("Field '{}' not found in schema for env '{}'.", key, env);
    }
    Ok(())
}

pub fn handle_schema_show(env: &str) -> Result<()> {
    let schema = load_schema(env)?;
    if schema.fields.is_empty() {
        println!("No schema defined for env '{}'.", env);
    } else {
        println!("Schema for '{}':", env);
        let mut keys: Vec<_> = schema.fields.keys().collect();
        keys.sort();
        for k in keys {
            let f = &schema.fields[k];
            let req = if f.required { "required" } else { "optional" };
            let desc = f.description.as_deref().unwrap_or("");
            println!("  {} [{}] ({}) {}", k, f.field_type, req, desc);
        }
    }
    Ok(())
}

pub fn handle_schema_validate(env: &str, secrets: &std::collections::HashMap<String, String>) -> Result<()> {
    let schema = load_schema(env)?;
    let keys: Vec<&str> = secrets.keys().map(|s| s.as_str()).collect();
    let missing = schema.check_required(&keys);
    let mut errors = 0usize;
    for m in &missing {
        eprintln!("Missing required field: '{}'", m);
        errors += 1;
    }
    for (k, v) in secrets {
        if let Err(e) = schema.validate_value(k, v) {
            eprintln!("Validation error: {}", e);
            errors += 1;
        }
    }
    if errors == 0 {
        println!("Schema validation passed for env '{}'.", env);
    } else {
        anyhow::bail!("{} schema validation error(s) found", errors);
    }
    Ok(())
}

fn parse_type(s: &str) -> Result<SchemaType> {
    match s {
        "string" => Ok(SchemaType::String),
        "number" => Ok(SchemaType::Number),
        "boolean" => Ok(SchemaType::Boolean),
        "url" => Ok(SchemaType::Url),
        "email" => Ok(SchemaType::Email),
        other => anyhow::bail!("Unknown type '{}'. Use: string, number, boolean, url, email", other),
    }
}
