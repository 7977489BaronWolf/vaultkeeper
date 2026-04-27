#[cfg(test)]
mod tests {
    use super::super::transform::*;

    #[test]
    fn test_from_str_known() {
        assert_eq!(Transform::from_str("uppercase"), Some(Transform::Uppercase));
        assert_eq!(Transform::from_str("LOWERCASE"), Some(Transform::Lowercase));
        assert_eq!(Transform::from_str("b64enc"), Some(Transform::Base64Encode));
        assert_eq!(Transform::from_str("base64decode"), Some(Transform::Base64Decode));
        assert_eq!(Transform::from_str("trim"), Some(Transform::Trim));
        assert_eq!(Transform::from_str("strip_newlines"), Some(Transform::StripNewlines));
    }

    #[test]
    fn test_from_str_unknown() {
        assert_eq!(Transform::from_str("reverse"), None);
        assert_eq!(Transform::from_str(""), None);
    }

    #[test]
    fn test_apply_uppercase() {
        let t = Transform::Uppercase;
        assert_eq!(t.apply("hello world").unwrap(), "HELLO WORLD");
    }

    #[test]
    fn test_apply_lowercase() {
        let t = Transform::Lowercase;
        assert_eq!(t.apply("HELLO").unwrap(), "hello");
    }

    #[test]
    fn test_apply_trim() {
        let t = Transform::Trim;
        assert_eq!(t.apply("  spaces  ").unwrap(), "spaces");
    }

    #[test]
    fn test_apply_strip_newlines() {
        let t = Transform::StripNewlines;
        assert_eq!(t.apply("line1\nline2\r\n").unwrap(), "line1line2");
    }

    #[test]
    fn test_base64_roundtrip() {
        let original = "secret-value-123";
        let encoded = Transform::Base64Encode.apply(original).unwrap();
        let decoded = Transform::Base64Decode.apply(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_apply_transforms_chain() {
        let transforms = vec![Transform::Trim, Transform::Uppercase];
        let result = apply_transforms("  hello  ", &transforms).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_apply_transforms_empty() {
        let result = apply_transforms("unchanged", &[]).unwrap();
        assert_eq!(result, "unchanged");
    }

    #[test]
    fn test_list_transforms() {
        let list = list_transforms();
        assert!(list.contains(&"uppercase"));
        assert!(list.contains(&"base64encode"));
        assert_eq!(list.len(), 6);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Transform::Uppercase.as_str(), "uppercase");
        assert_eq!(Transform::Base64Encode.as_str(), "base64encode");
    }
}
