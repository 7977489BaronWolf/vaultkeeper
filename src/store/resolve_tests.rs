#[cfg(test)]
mod tests {
    use super::super::resolve::*;
    use std::collections::HashMap;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn test_interpolate_simple() {
        let secrets = map(&[("HOST", "localhost"), ("PORT", "5432")]);
        let result = interpolate("${HOST}:${PORT}", &secrets);
        assert_eq!(result, "localhost:5432");
    }

    #[test]
    fn test_interpolate_missing_var() {
        let secrets = map(&[("HOST", "localhost")]);
        let result = interpolate("${HOST}:${PORT}", &secrets);
        assert_eq!(result, "localhost:${PORT}");
    }

    #[test]
    fn test_interpolate_no_refs() {
        let secrets = map(&[("HOST", "localhost")]);
        let result = interpolate("plain-value", &secrets);
        assert_eq!(result, "plain-value");
    }

    #[test]
    fn test_resolve_references() {
        let secrets = map(&[
            ("BASE_URL", "http://${HOST}:${PORT}"),
            ("HOST", "example.com"),
            ("PORT", "8080"),
        ]);
        let resolved = resolve_references(&secrets);
        assert_eq!(resolved["BASE_URL"], "http://example.com:8080");
        assert_eq!(resolved["HOST"], "example.com");
    }

    #[test]
    fn test_detect_cycles_none() {
        let secrets = map(&[
            ("A", "hello"),
            ("B", "${A}-world"),
        ]);
        assert!(detect_cycles(&secrets).is_ok());
    }

    #[test]
    fn test_detect_cycles_direct() {
        let secrets = map(&[("A", "${A}") ]);
        assert!(detect_cycles(&secrets).is_err());
    }

    #[test]
    fn test_detect_cycles_indirect() {
        let secrets = map(&[
            ("A", "${B}"),
            ("B", "${A}"),
        ]);
        assert!(detect_cycles(&secrets).is_err());
    }

    #[test]
    fn test_extract_multiple_refs() {
        let secrets = map(&[
            ("X", "1"), ("Y", "2"), ("Z", "3"),
        ]);
        let result = interpolate("${X}-${Y}-${Z}", &secrets);
        assert_eq!(result, "1-2-3");
    }
}
