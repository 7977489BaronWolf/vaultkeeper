#[cfg(test)]
mod tests {
    use super::super::secret_type::*;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_display_builtin_types() {
        assert_eq!(SecretType::Plain.to_string(), "plain");
        assert_eq!(SecretType::ApiKey.to_string(), "api_key");
        assert_eq!(SecretType::Password.to_string(), "password");
        assert_eq!(SecretType::Token.to_string(), "token");
        assert_eq!(SecretType::ConnectionString.to_string(), "connection_string");
        assert_eq!(SecretType::Certificate.to_string(), "certificate");
        assert_eq!(SecretType::PrivateKey.to_string(), "private_key");
        assert_eq!(SecretType::Url.to_string(), "url");
    }

    #[test]
    fn test_display_custom_type() {
        let t = SecretType::Custom("oauth_secret".to_string());
        assert_eq!(t.to_string(), "custom:oauth_secret");
    }

    #[test]
    fn test_from_str_builtin() {
        assert_eq!(SecretType::from_str("plain").unwrap(), SecretType::Plain);
        assert_eq!(SecretType::from_str("api_key").unwrap(), SecretType::ApiKey);
        assert_eq!(SecretType::from_str("password").unwrap(), SecretType::Password);
        assert_eq!(SecretType::from_str("token").unwrap(), SecretType::Token);
        assert_eq!(SecretType::from_str("url").unwrap(), SecretType::Url);
    }

    #[test]
    fn test_from_str_custom() {
        let t = SecretType::from_str("custom:webhook_secret").unwrap();
        assert_eq!(t, SecretType::Custom("webhook_secret".to_string()));
    }

    #[test]
    fn test_from_str_unknown_returns_error() {
        let result = SecretType::from_str("unknown_thing");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown_thing"));
    }

    #[test]
    fn test_annotate_and_get_type() {
        let mut store: HashMap<String, String> = HashMap::new();
        store.insert("MY_TOKEN".to_string(), "abc123".to_string());
        annotate(&mut store, "MY_TOKEN", &SecretType::Token);
        let t = get_type(&store, "MY_TOKEN");
        assert_eq!(t, SecretType::Token);
    }

    #[test]
    fn test_get_type_defaults_to_plain() {
        let store: HashMap<String, String> = HashMap::new();
        let t = get_type(&store, "MISSING_KEY");
        assert_eq!(t, SecretType::Plain);
    }

    #[test]
    fn test_builtin_types_list() {
        let types = builtin_types();
        assert!(types.contains(&"api_key"));
        assert!(types.contains(&"password"));
        assert_eq!(types.len(), 8);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let t = SecretType::ConnectionString;
        let s = t.to_string();
        let parsed = SecretType::from_str(&s).unwrap();
        assert_eq!(parsed, t);
    }
}
