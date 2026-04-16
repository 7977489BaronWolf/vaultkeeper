use std::io::{Read, Write};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

/// Compress bytes using gzip
pub fn compress(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

/// Decompress gzip-compressed bytes
pub fn decompress(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}

/// Returns true if the data appears to be gzip-compressed
pub fn is_compressed(data: &[u8]) -> bool {
    data.starts_with(&[0x1f, 0x8b])
}

/// Compress if not already compressed
pub fn ensure_compressed(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if is_compressed(data) {
        Ok(data.to_vec())
    } else {
        compress(data)
    }
}

/// Decompress if compressed, otherwise return as-is
pub fn ensure_decompressed(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if is_compressed(data) {
        decompress(data)
    } else {
        Ok(data.to_vec())
    }
}
