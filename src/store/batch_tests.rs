#[cfg(test)]
mod tests {
    use super::super::batch::*;
    use std::collections::HashMap;

    fn base_store() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("FOO".into(), "bar".into());
        m.insert("BAZ".into(), "qux".into());
        m
    }

    #[test]
    fn test_set_op_inserts_value() {
        let mut store = base_store();
        let ops = vec![BatchOp::Set {
            key: "NEW_KEY".into(),
            value: "hello".into(),
        }];
        let result = apply_batch(&mut store, ops);
        assert!(result.is_ok());
        assert_eq!(store.get("NEW_KEY").map(|s| s.as_str()), Some("hello"));
        assert!(result.succeeded.contains(&"NEW_KEY".to_string()));
    }

    #[test]
    fn test_delete_op_removes_existing_key() {
        let mut store = base_store();
        let ops = vec![BatchOp::Delete { key: "FOO".into() }];
        let result = apply_batch(&mut store, ops);
        assert!(result.is_ok());
        assert!(!store.contains_key("FOO"));
    }

    #[test]
    fn test_delete_nonexistent_key_fails() {
        let mut store = base_store();
        let ops = vec![BatchOp::Delete {
            key: "MISSING".into(),
        }];
        let result = apply_batch(&mut store, ops);
        assert!(!result.is_ok());
        assert_eq!(result.failed.len(), 1);
    }

    #[test]
    fn test_set_empty_key_fails() {
        let mut store = base_store();
        let ops = vec![BatchOp::Set {
            key: "  ".into(),
            value: "val".into(),
        }];
        let result = apply_batch(&mut store, ops);
        assert!(!result.is_ok());
    }

    #[test]
    fn test_parse_batch_ops_valid() {
        let input = "SET FOO=bar\nDELETE BAZ\n# comment\nUNSET QUX";
        let ops = parse_batch_ops(input).unwrap();
        assert_eq!(ops.len(), 3);
        assert_eq!(
            ops[0],
            BatchOp::Set {
                key: "FOO".into(),
                value: "bar".into()
            }
        );
        assert_eq!(ops[1], BatchOp::Delete { key: "BAZ".into() });
        assert_eq!(ops[2], BatchOp::Unset { key: "QUX".into() });
    }

    #[test]
    fn test_parse_batch_ops_invalid_set() {
        let input = "SET NOEQUALS";
        assert!(parse_batch_ops(input).is_err());
    }

    #[test]
    fn test_parse_batch_ops_unknown_op() {
        let input = "REPLACE FOO=bar";
        assert!(parse_batch_ops(input).is_err());
    }

    #[test]
    fn test_mixed_success_and_failure() {
        let mut store = base_store();
        let ops = vec![
            BatchOp::Set {
                key: "NEW".into(),
                value: "v".into(),
            },
            BatchOp::Delete { key: "GONE".into() },
        ];
        let result = apply_batch(&mut store, ops);
        assert_eq!(result.succeeded.len(), 1);
        assert_eq!(result.failed.len(), 1);
    }
}
