#[cfg(test)]
mod tests {
    use super::super::fmt::*;
    use std::collections::HashMap;

    fn sample() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("APP_ENV".to_string(), "production".to_string());
        m.insert("DB_URL".to_string(), "postgres://localhost/db".to_string());
        m
    }

    #[test]
    fn test_from_str_valid() {
        assert_eq!(FmtStyle::from_str("dotenv"), Some(FmtStyle::Dotenv));
        assert_eq!(FmtStyle::from_str(".env"), Some(FmtStyle::Dotenv));
        assert_eq!(FmtStyle::from_str("json"), Some(FmtStyle::Json));
        assert_eq!(FmtStyle::from_str("shell"), Some(FmtStyle::Shell));
        assert_eq!(FmtStyle::from_str("sh"), Some(FmtStyle::Shell));
        assert_eq!(FmtStyle::from_str("csv"), Some(FmtStyle::Csv));
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(FmtStyle::from_str("xml"), None);
        assert_eq!(FmtStyle::from_str(""), None);
    }

    #[test]
    fn test_format_dotenv() {
        let out = format_vars(&sample(), &FmtStyle::Dotenv);
        assert!(out.contains("APP_ENV=production"));
        assert!(out.contains("DB_URL=postgres://localhost/db"));
    }

    #[test]
    fn test_format_shell() {
        let out = format_vars(&sample(), &FmtStyle::Shell);
        assert!(out.contains("export APP_ENV=\"production\""));
        assert!(out.contains("export DB_URL=\""));
    }

    #[test]
    fn test_format_json() {
        let out = format_vars(&sample(), &FmtStyle::Json);
        assert!(out.starts_with('{'));
        assert!(out.ends_with('}'));
        assert!(out.contains("\"APP_ENV\""));
        assert!(out.contains("\"production\""));
    }

    #[test]
    fn test_format_csv() {
        let out = format_vars(&sample(), &FmtStyle::Csv);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines[0], "key,value");
        assert!(out.contains("APP_ENV,production"));
    }

    #[test]
    fn test_format_empty() {
        let empty = HashMap::new();
        assert_eq!(format_vars(&empty, &FmtStyle::Dotenv), "");
        assert_eq!(format_vars(&empty, &FmtStyle::Json), "{}");
    }
}
