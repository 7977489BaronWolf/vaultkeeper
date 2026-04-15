#[cfg(test)]
mod tests {
    use super::super::tags::*;

    const SAMPLE: &str = "\
# @tags: production,database
DB_HOST=prod.db.example.com
# @tags: production
API_KEY=secret123
# @tags: development
DEBUG=true
PLAIN_VAR=no_tags
";

    #[test]
    fn test_parse_tags_from_comment_basic() {
        let tags = parse_tags_from_comment("# @tags: production,database");
        assert_eq!(tags, vec!["production", "database"]);
    }

    #[test]
    fn test_parse_tags_from_comment_single() {
        let tags = parse_tags_from_comment("# @tags: staging");
        assert_eq!(tags, vec!["staging"]);
    }

    #[test]
    fn test_parse_tags_from_comment_empty() {
        let tags = parse_tags_from_comment("# some other comment");
        assert!(tags.is_empty());
    }

    #[test]
    fn test_parse_tags_normalizes_case() {
        let tags = parse_tags_from_comment("# @tags: PROD,Dev");
        assert_eq!(tags, vec!["prod", "dev"]);
    }

    #[test]
    fn test_extract_tags_maps_keys() {
        let map = extract_tags(SAMPLE);
        assert_eq!(map.get("DB_HOST").unwrap(), &vec!["production", "database"]);
        assert_eq!(map.get("API_KEY").unwrap(), &vec!["production"]);
        assert_eq!(map.get("DEBUG").unwrap(), &vec!["development"]);
        assert!(!map.contains_key("PLAIN_VAR"));
    }

    #[test]
    fn test_filter_by_tag_production() {
        let results = filter_by_tag(SAMPLE, "production");
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|l| l.contains("DB_HOST")));
        assert!(results.iter().any(|l| l.contains("API_KEY")));
    }

    #[test]
    fn test_filter_by_tag_development() {
        let results = filter_by_tag(SAMPLE, "development");
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("DEBUG"));
    }

    #[test]
    fn test_filter_by_tag_no_match() {
        let results = filter_by_tag(SAMPLE, "nonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_filter_by_tag_case_insensitive() {
        let results = filter_by_tag(SAMPLE, "PRODUCTION");
        assert_eq!(results.len(), 2);
    }
}
