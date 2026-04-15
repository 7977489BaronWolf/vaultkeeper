#[cfg(test)]
mod tests {
    use super::super::validate::*;

    #[test]
    fn test_valid_key() {
        assert!(validate_key("MY_VAR").is_ok());
        assert!(validate_key("DB_HOST_1").is_ok());
        assert!(validate_key("A").is_ok());
    }

    #[test]
    fn test_empty_key() {
        assert_eq!(validate_key(""), Err(ValidationError::EmptyKey));
    }

    #[test]
    fn test_invalid_key_lowercase() {
        let err = validate_key("my_var");
        assert!(matches!(err, Err(ValidationError::InvalidKeyChar(_))));
    }

    #[test]
    fn test_invalid_key_hyphen() {
        let err = validate_key("MY-VAR");
        assert!(matches!(err, Err(ValidationError::InvalidKeyChar(_))));
    }

    #[test]
    fn test_valid_value() {
        assert!(validate_value("somevalue").is_ok());
        assert!(validate_value(" ").is_ok());
    }

    #[test]
    fn test_empty_value() {
        assert_eq!(validate_value(""), Err(ValidationError::EmptyValue));
    }

    #[test]
    fn test_validate_entry_ok() {
        assert!(validate_entry("DB_HOST", "localhost").is_ok());
    }

    #[test]
    fn test_validate_entry_bad_key() {
        assert!(matches!(
            validate_entry("db_host", "localhost"),
            Err(ValidationError::InvalidKeyChar(_))
        ));
    }

    #[test]
    fn test_no_duplicates_ok() {
        let entries = vec![
            ("KEY_A".to_string(), "val1".to_string()),
            ("KEY_B".to_string(), "val2".to_string()),
        ];
        assert!(validate_no_duplicates(&entries).is_ok());
    }

    #[test]
    fn test_duplicate_key_detected() {
        let entries = vec![
            ("KEY_A".to_string(), "val1".to_string()),
            ("KEY_A".to_string(), "val2".to_string()),
        ];
        assert_eq!(
            validate_no_duplicates(&entries),
            Err(ValidationError::DuplicateKey("KEY_A".to_string()))
        );
    }

    #[test]
    fn test_validation_error_display() {
        assert_eq!(ValidationError::EmptyKey.to_string(), "Key must not be empty");
        assert_eq!(ValidationError::EmptyValue.to_string(), "Value must not be empty");
        assert_eq!(
            ValidationError::DuplicateKey("X".to_string()).to_string(),
            "Duplicate key: 'X'"
        );
    }
}
