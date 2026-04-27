use crate::store::redact::{is_sensitive_key, redact_env};
use std::collections::HashSet;

/// Options controlling redaction output
pub struct RedactOptions {
    pub show_keys: bool,
    pub force_redact: HashSet<String>,
}

impl Default for RedactOptions {
    fn default() -> Self {
        Self {
            show_keys: false,
            force_redact: HashSet::new(),
        }
    }
}

/// Print env vars with sensitive values redacted
pub fn print_redacted(env: &[(String, String)], opts: &RedactOptions) {
    let pairs: Vec<(String, String)> = env
        .iter()
        .map(|(k, v)| {
            let should_redact = opts.force_redact.contains(k) || is_sensitive_key(k);
            let display_val = if should_redact {
                crate::store::redact::redact_value(v)
            } else {
                v.clone()
            };
            (k.clone(), display_val)
        })
        .collect();

    for (k, v) in &pairs {
        if opts.show_keys {
            println!("{k}={v}  # redacted");
        } else {
            println!("{k}={v}");
        }
    }
}

/// Summarise how many keys would be redacted in a given env list
pub fn redact_summary(env: &[(String, String)]) -> String {
    let total = env.len();
    let sensitive = env.iter().filter(|(k, _)| is_sensitive_key(k)).count();
    format!(
        "{} of {} keys contain sensitive values and will be redacted.",
        sensitive, total
    )
}

/// Handle the `vaultkeeper redact` subcommand
pub fn handle_redact(env: &[(String, String)], show_keys: bool) {
    let opts = RedactOptions {
        show_keys,
        force_redact: HashSet::new(),
    };
    let summary = redact_summary(env);
    eprintln!("[redact] {summary}");
    print_redacted(env, &opts);
}
