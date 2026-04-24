#[cfg(test)]
mod tests {
    use super::super::schema::*;
    use std::collections::HashMap;

    #[test]
    fn test_handle_schema_validate_passes() {
        let mut secrets = HashMap::new();
        secrets.insert("NAME".to_string(), "alice".to_string());
        // No schema defined for this fake env, should pass trivially
        // We test the validate logic via the schema store directly
        let result = handle_schema_validate("__test_empty__", &secrets);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_type_valid() {
        // Access via the public function indirectly by calling add which uses it
        // We test round-trip by calling add and then show would not panic
        let result = handle_schema_add(
            "__test_schema__",
            "MY_KEY",
            "number",
            false,
            Some("A number field".to_string()),
        );
        assert!(result.is_ok());
        // cleanup
        let _ = std::fs::remove_file(".vaultkeeper/__test_schema__.schema.json");
    }

    #[test]
    fn test_handle_schema_add_invalid_type() {
        let result = handle_schema_add(
            "__test_schema2__",
            "BAD",
            "invalid_type",
            false,
            None,
        );
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("Unknown type"));
    }

    #[test]
    fn test_handle_schema_remove_nonexistent() {
        // Should not error, just print message
        let result = handle_schema_remove("__test_noschema__", "MISSING_KEY");
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_schema_validate_missing_required() {
        use crate::store::schema::{Schema, SchemaField, SchemaType};
        use std::path::PathBuf;
        // Manually create a schema file for the test env
        let env = "__test_validate_req__";
        let mut schema = Schema::new();
        schema.add_field("DB_URL".into(), SchemaField {
            field_type: SchemaType::String,
            required: true,
            description: None,
            pattern: None,
        });
        let path = PathBuf::from(format!(".vaultkeeper/{}.schema.json", env));
        std::fs::create_dir_all(".vaultkeeper").unwrap();
        std::fs::write(&path, serde_json::to_string(&schema).unwrap()).unwrap();

        let secrets: HashMap<String, String> = HashMap::new();
        let result = handle_schema_validate(env, &secrets);
        assert!(result.is_err());
        let _ = std::fs::remove_file(&path);
    }
}
