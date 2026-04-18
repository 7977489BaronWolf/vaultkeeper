use std::collections::HashMap;

/// Format styles for exporting env vars
#[derive(Debug, Clone, PartialEq)]
pub enum FmtStyle {
    Dotenv,
    Json,
    Shell,
    Csv,
}

impl FmtStyle {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "dotenv" | ".env" => Some(FmtStyle::Dotenv),
            "json" => Some(FmtStyle::Json),
            "shell" | "sh" => Some(FmtStyle::Shell),
            "csv" => Some(FmtStyle::Csv),
            _ => None,
        }
    }
}

pub fn format_vars(vars: &HashMap<String, String>, style: &FmtStyle) -> String {
    let mut keys: Vec<&String> = vars.keys().collect();
    keys.sort();

    match style {
        FmtStyle::Dotenv => keys
            .iter()
            .map(|k| format!("{}={}", k, vars[*k]))
            .collect::<Vec<_>>()
            .join("\n"),

        FmtStyle::Json => {
            let pairs: Vec<String> = keys
                .iter()
                .map(|k| format!("  \"{}\": \"{}\"", k, vars[*k].replace('"', "\\\"")))
                .collect();
            format!("{{{}}}", if pairs.is_empty() { String::new() } else { format!("\n{}\n", pairs.join(",\n")) })
        }

        FmtStyle::Shell => keys
            .iter()
            .map(|k| format!("export {}=\"{}\"", k, vars[*k]))
            .collect::<Vec<_>>()
            .join("\n"),

        FmtStyle::Csv => {
            let mut lines = vec!["key,value".to_string()];
            lines.extend(keys.iter().map(|k| format!("{},{}", k, vars[*k])));
            lines.join("\n")
        }
    }
}
