use std::collections::HashMap;

/// Applies a named transformation to a secret value.
#[derive(Debug, Clone, PartialEq)]
pub enum Transform {
    Uppercase,
    Lowercase,
    Base64Encode,
    Base64Decode,
    Trim,
    StripNewlines,
}

impl Transform {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "uppercase" => Some(Transform::Uppercase),
            "lowercase" => Some(Transform::Lowercase),
            "base64encode" | "b64enc" => Some(Transform::Base64Encode),
            "base64decode" | "b64dec" => Some(Transform::Base64Decode),
            "trim" => Some(Transform::Trim),
            "stripnewlines" | "strip_newlines" => Some(Transform::StripNewlines),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Transform::Uppercase => "uppercase",
            Transform::Lowercase => "lowercase",
            Transform::Base64Encode => "base64encode",
            Transform::Base64Decode => "base64decode",
            Transform::Trim => "trim",
            Transform::StripNewlines => "stripnewlines",
        }
    }

    pub fn apply(&self, value: &str) -> Result<String, String> {
        match self {
            Transform::Uppercase => Ok(value.to_uppercase()),
            Transform::Lowercase => Ok(value.to_lowercase()),
            Transform::Base64Encode => {
                use std::fmt::Write;
                let encoded = base64_encode(value.as_bytes());
                Ok(encoded)
            }
            Transform::Base64Decode => {
                base64_decode(value)
                    .map_err(|e| format!("base64 decode error: {}", e))
            }
            Transform::Trim => Ok(value.trim().to_string()),
            Transform::StripNewlines => Ok(value.replace('\n', "").replace('\r', "")),
        }
    }
}

pub fn apply_transforms(value: &str, transforms: &[Transform]) -> Result<String, String> {
    let mut result = value.to_string();
    for t in transforms {
        result = t.apply(&result)?;
    }
    Ok(result)
}

pub fn list_transforms() -> Vec<&'static str> {
    vec!["uppercase", "lowercase", "base64encode", "base64decode", "trim", "stripnewlines"]
}

fn base64_encode(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
        out.push(CHARS[(b0 >> 2)] as char);
        out.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        out.push(if chunk.len() > 1 { CHARS[((b1 & 15) << 2) | (b2 >> 6)] as char } else { '=' });
        out.push(if chunk.len() > 2 { CHARS[b2 & 63] as char } else { '=' });
    }
    out
}

fn base64_decode(s: &str) -> Result<String, String> {
    let s = s.trim_end_matches('=');
    let mut bytes = Vec::new();
    let chars: Vec<u8> = s.bytes().collect();
    for chunk in chars.chunks(4) {
        let vals: Vec<u8> = chunk.iter().map(|&c| decode_char(c)).collect::<Result<Vec<_>, _>>()?;
        if vals.len() >= 2 {
            bytes.push((vals[0] << 2) | (vals[1] >> 4));
        }
        if vals.len() >= 3 {
            bytes.push((vals[1] << 4) | (vals[2] >> 2));
        }
        if vals.len() == 4 {
            bytes.push((vals[2] << 6) | vals[3]);
        }
    }
    String::from_utf8(bytes).map_err(|e| e.to_string())
}

fn decode_char(c: u8) -> Result<u8, String> {
    match c {
        b'A'..=b'Z' => Ok(c - b'A'),
        b'a'..=b'z' => Ok(c - b'a' + 26),
        b'0'..=b'9' => Ok(c - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(format!("invalid base64 char: {}", c as char)),
    }
}
