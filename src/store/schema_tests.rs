#[cfg(test)]
mod tests {
    use super::super::schema::*;

    fn make_field(t: SchemaType, required: bool) -> SchemaField {
        SchemaField {
            field_type: t,
            required,
            description: None,
            pattern: None,
        }
    }

    #[test]
    fn test_add_and_remove_field() {
        let mut schema = Schema::new();
        schema.add_field("PORT".into(), make_field(SchemaType::Number, true));
        assert!(schema.fields.contains_key("PORT"));
        assert!(schema.remove_field("PORT"));
        assert!(!schema.fields.contains_key("PORT"));
        assert!(!schema.remove_field("PORT"));
    }

    #[test]
    fn test_validate_number() {
        let mut schema = Schema::new();
        schema.add_field("PORT".into(), make_field(SchemaType::Number, false));
        assert!(schema.validate_value("PORT", "8080").is_ok());
        assert!(schema.validate_value("PORT", "abc").is_err());
    }

    #[test]
    fn test_validate_boolean() {
        let mut schema = Schema::new();
        schema.add_field("DEBUG".into(), make_field(SchemaType::Boolean, false));
        assert!(schema.validate_value("DEBUG", "true").is_ok());
        assert!(schema.validate_value("DEBUG", "false").is_ok());
        assert!(schema.validate_value("DEBUG", "yes").is_ok());
        assert!(schema.validate_value("DEBUG", "maybe").is_err());
    }

    #[test]
    fn test_validate_url() {
        let mut schema = Schema::new();
        schema.add_field("API_URL".into(), make_field(SchemaType::Url, false));
        assert!(schema.validate_value("API_URL", "https://example.com").is_ok());
        assert!(schema.validate_value("API_URL", "http://localhost").is_ok());
        assert!(schema.validate_value("API_URL", "ftp://bad").is_err());
    }

    #[test]
    fn test_validate_email() {
        let mut schema = Schema::new();
        schema.add_field("CONTACT".into(), make_field(SchemaType::Email, false));
        assert!(schema.validate_value("CONTACT", "user@example.com").is_ok());
        assert!(schema.validate_value("CONTACT", "notanemail").is_err());
    }

    #[test]
    fn test_check_required() {
        let mut schema = Schema::new();
        schema.add_field("DB_URL".into(), make_field(SchemaType::String, true));
        schema.add_field("PORT".into(), make_field(SchemaType::Number, true));
        schema.add_field("DEBUG".into(), make_field(SchemaType::Boolean, false));
        let present = vec!["PORT"];
        let missing = schema.check_required(&present);
        assert!(missing.contains(&"DB_URL".to_string()));
        assert!(!missing.contains(&"PORT".to_string()));
        assert!(!missing.contains(&"DEBUG".to_string()));
    }

    #[test]
    fn test_unknown_key_skipped() {
        let schema = Schema::new();
        assert!(schema.validate_value("UNKNOWN", "anything").is_ok());
    }
}
