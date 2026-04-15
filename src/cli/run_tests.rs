#[cfg(test)]
mod tests {
    use super::super::run::parse_env_contents_pub;
    use std::collections::HashMap;

    // Re-export the private fn for testing via a thin wrapper in run.rs
    #[test]
    fn test_parse_basic_env() {
        let contents = "FOO=bar\nBAZ=qux\n";
        let result = parse_env_contents_pub(contents).unwrap();
        assert_eq!(result.get("FOO"), Some(&"bar".to_string()));
        assert_eq!(result.get("BAZ"), Some(&"qux".to_string()));
    }

    #[test]
    fn test_parse_skips_comments_and_blank_lines() {
        let contents = "# This is a comment\n\nKEY=value\n";
        let result = parse_env_contents_pub(contents).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("KEY"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_value_with_equals_sign() {
        let contents = "TOKEN=abc=def=ghi\n";
        let result = parse_env_contents_pub(contents).unwrap();
        assert_eq!(result.get("TOKEN"), Some(&"abc=def=ghi".to_string()));
    }

    #[test]
    fn test_parse_trims_whitespace() {
        let contents = "  DB_HOST = localhost  \n";
        let result = parse_env_contents_pub(contents).unwrap();
        assert_eq!(result.get("DB_HOST"), Some(&"localhost".to_string()));
    }

    #[test]
    fn test_parse_invalid_line_returns_error() {
        let contents = "INVALID_LINE_WITHOUT_EQUALS\n";
        let result = parse_env_contents_pub(contents);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_contents() {
        let result = parse_env_contents_pub("").unwrap();
        assert!(result.is_empty());
    }
}
