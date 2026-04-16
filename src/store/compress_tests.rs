#[cfg(test)]
mod tests {
    use super::super::compress::*;

    #[test]
    fn test_compress_decompress_roundtrip() {
        let original = b"DATABASE_URL=postgres://localhost/mydb\nSECRET_KEY=abc123";
        let compressed = compress(original).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_compressed_is_smaller_for_repetitive_data() {
        let data = "KEY=VALUE\n".repeat(100);
        let compressed = compress(data.as_bytes()).unwrap();
        assert!(compressed.len() < data.len());
    }

    #[test]
    fn test_is_compressed_detects_gzip_magic_bytes() {
        let data = b"plain text";
        assert!(!is_compressed(data));
        let compressed = compress(data).unwrap();
        assert!(is_compressed(&compressed));
    }

    #[test]
    fn test_ensure_compressed_idempotent() {
        let data = b"FOO=BAR";
        let once = ensure_compressed(data).unwrap();
        let twice = ensure_compressed(&once).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn test_ensure_decompressed_on_plain_data() {
        let data = b"FOO=BAR";
        let result = ensure_decompressed(data).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_ensure_decompressed_on_compressed_data() {
        let data = b"FOO=BAR";
        let compressed = compress(data).unwrap();
        let result = ensure_decompressed(&compressed).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_empty_input() {
        let compressed = compress(b"").unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(decompressed, b"");
    }
}
