use crate::store::batch::{apply_batch, parse_batch_ops, BatchResult};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn run_batch_file(
    store: &mut HashMap<String, String>,
    path: &PathBuf,
    dry_run: bool,
) -> Result<BatchResult, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read batch file: {}", e))?;
    let ops = parse_batch_ops(&content)?;

    if dry_run {
        println!("[dry-run] {} operation(s) parsed:", ops.len());
        for op in &ops {
            println!("  {:?}", op);
        }
        return Ok(crate::store::batch::BatchResult::new());
    }

    let result = apply_batch(store, ops);
    print_batch_result(&result);
    Ok(result)
}

fn print_batch_result(result: &BatchResult) {
    for key in &result.succeeded {
        println!("  ✓ {}", key);
    }
    for (key, reason) in &result.failed {
        eprintln!("  ✗ {}: {}", key, reason);
    }
    println!(
        "\nbatch complete: {} succeeded, {} failed",
        result.succeeded.len(),
        result.failed.len()
    );
}

pub fn handle_batch(path: &str, dry_run: bool) -> anyhow::Result<()> {
    let path = PathBuf::from(path);
    if !path.exists() {
        anyhow::bail!("batch file not found: {}", path.display());
    }

    // In a real integration this would load/save the encrypted store.
    // Here we demonstrate with an in-memory stub.
    let mut store: HashMap<String, String> = HashMap::new();

    let result = run_batch_file(&mut store, &path, dry_run)
        .map_err(|e| anyhow::anyhow!(e))?;

    if !result.is_ok() && !dry_run {
        anyhow::bail!(
            "{} operation(s) failed",
            result.failed.len()
        );
    }

    Ok(())
}
