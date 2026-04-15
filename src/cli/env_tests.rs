#[cfg(test)]
mod tests {
    use super::super::env::{format_env_contents, parse_env_contents};
    use std::collections::HashMap;

    #[test]
    fn test_parse_simple_vars() {
        let input = "FOO=bar\nBAZ=qux\n";
        let result = parse_env_contents(input);
        assert_eq!(result.get("FOO"), Some(&"bar".to_string()));
        assert_eq!(result.get("BAZ"), Some(&"qux".to_string()));
    }

    #[test]
    fn test_parse_skips_comments_and_blanks() {
        let input = "# This is a comment\n\nKEY=value\n";
        let result = parse_env_contents(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("KEY"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_strips_quotes() {
        let input = "SECRET=\"my secret value\"\n";
        let result = parse_env_contents(input);
        assert_eq!(result.get("SECRET"), Some(&"my secret value".to_string()));
    }

    #[test]
    fn test_parse_trims_whitespace() {
        let input = "  KEY  =  value  \n";
        let result = parse_env_contents(input);
        assert_eq!(result.get("KEY"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_ignores_lines_without_equals() {
        let input = "INVALID_LINE\nVALID=yes\n";
        let result = parse_env_contents(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("VALID"), Some(&"yes".to_string()));
    }

    #[test]
    fn test_format_env_contents_sorted() {
        let mut vars = HashMap::new();
        vars.insert("ZEBRA".to_string(), "last".to_string());
        vars.insert("ALPHA".to_string(), "first".to_string());
        let output = format_env_contents(&vars);
        let lines: Vec<&str> = output.trim().lines().collect();
        assert_eq!(lines[0], "ALPHA=first");
        assert_eq!(lines[1], "ZEBRA=last");
    }

    #[test]
    fn test_format_quotes_values_with_spaces() {
        let mut vars = HashMap::new();
        vars.insert("MSG".to_string(), "hello world".to_string());
        let output = format_env_contents(&vars);
        assert!(output.contains("MSG=\"hello world\""));
    }

    #[test]
    fn test_roundtrip() {
        let input = "API_KEY=abc123\nDB_URL=postgres://localhost/mydb\n";
        let parsed = parse_env_contents(input);
        let formatted = format_env_contents(&parsed);
        let reparsed = parse_env_contents(&formatted);
        assert_eq!(parsed, reparsed);
    }
}
