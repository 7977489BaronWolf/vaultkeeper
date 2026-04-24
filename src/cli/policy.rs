use crate::store::policy::{Policy, PolicyRule, PolicyStore};
use anyhow::{anyhow, Result};

pub fn run_policy_add(store: &mut PolicyStore, name: &str, rule_str: &str) -> Result<()> {
    let rule = parse_rule(rule_str)?;
    let policy = store
        .policies
        .entry(name.to_string())
        .or_insert_with(|| Policy::new(name));
    policy.add_rule(rule);
    println!("Policy '{}' updated with rule: {}", name, rule_str);
    Ok(())
}

pub fn run_policy_remove(store: &mut PolicyStore, name: &str) -> Result<()> {
    if store.remove_policy(name) {
        println!("Policy '{}' removed.", name);
        Ok(())
    } else {
        Err(anyhow!("Policy '{}' not found.", name))
    }
}

pub fn run_policy_list(store: &PolicyStore) {
    let policies = store.list();
    if policies.is_empty() {
        println!("No policies defined.");
        return;
    }
    for policy in policies {
        println!("Policy: {}", policy.name);
        for rule in &policy.rules {
            println!("  - {:?}", rule);
        }
    }
}

pub fn run_policy_check(
    store: &PolicyStore,
    key: &str,
    value: &str,
    namespace: &str,
) -> Result<()> {
    let errors = store.evaluate_all(key, value, namespace);
    if errors.is_empty() {
        println!("OK: '{}' passes all policies.", key);
        Ok(())
    } else {
        for e in &errors {
            eprintln!("VIOLATION: {}", e);
        }
        Err(anyhow!("Policy violations found for key '{}'.", key))
    }
}

fn parse_rule(rule_str: &str) -> Result<PolicyRule> {
    if let Some(rest) = rule_str.strip_prefix("deny-keys:") {
        let keys = rest.split(',').map(|s| s.trim().to_string()).collect();
        return Ok(PolicyRule::DenyKeys(keys));
    }
    if let Some(rest) = rule_str.strip_prefix("require-prefix:") {
        return Ok(PolicyRule::RequirePrefix(rest.trim().to_string()));
    }
    if let Some(rest) = rule_str.strip_prefix("max-value-length:") {
        let n: usize = rest.trim().parse().map_err(|_| anyhow!("Invalid length value"))?;
        return Ok(PolicyRule::MaxValueLength(n));
    }
    if let Some(rest) = rule_str.strip_prefix("allowed-namespaces:") {
        let ns = rest.split(',').map(|s| s.trim().to_string()).collect();
        return Ok(PolicyRule::AllowedNamespaces(ns));
    }
    Err(anyhow!("Unknown rule format: '{}'", rule_str))
}
