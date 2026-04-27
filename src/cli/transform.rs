use crate::store::transform::{Transform, apply_transforms, list_transforms};

pub fn run_transform(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: vaultkeeper transform <value> <transform1> [transform2 ...]".to_string());
    }

    if args.len() == 1 && (args[0] == "--list" || args[0] == "-l") {
        println!("Available transforms:");
        for t in list_transforms() {
            println!("  {}", t);
        }
        return Ok(());
    }

    if args.len() < 2 {
        return Err("Usage: vaultkeeper transform <value> <transform1> [transform2 ...]".to_string());
    }

    let value = &args[0];
    let transform_names = &args[1..];

    let mut transforms = Vec::new();
    for name in transform_names {
        match Transform::from_str(name) {
            Some(t) => transforms.push(t),
            None => {
                return Err(format!(
                    "Unknown transform '{}'. Run `vaultkeeper transform --list` to see available transforms.",
                    name
                ));
            }
        }
    }

    match apply_transforms(value, &transforms) {
        Ok(result) => {
            println!("{}", result);
            Ok(())
        }
        Err(e) => Err(format!("Transform failed: {}", e)),
    }
}

pub fn describe_transform(name: &str) -> Option<&'static str> {
    match name {
        "uppercase" => Some("Convert all characters to uppercase"),
        "lowercase" => Some("Convert all characters to lowercase"),
        "base64encode" | "b64enc" => Some("Encode value as base64"),
        "base64decode" | "b64dec" => Some("Decode value from base64"),
        "trim" => Some("Remove leading and trailing whitespace"),
        "stripnewlines" | "strip_newlines" => Some("Remove all newline characters"),
        _ => None,
    }
}
