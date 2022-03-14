/*
Copyright (c) 2022 Daniel C Brotsky.
All rights reserved.

See the README.md, LICENSE-MIT, and LICENSE-APACHE files for license details.
 */
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use serde_json::{from_str, json, Value};
use std::io::{stdin, Read};

pub fn base64url_decode_str(input: &str) -> String {
    match decode_config(input, URL_SAFE_NO_PAD) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(err) => format!("Decode error: {:?}", err),
    }
}

pub fn base64url_decode_filter() {
    let mut input = String::new();
    match stdin().read_to_string(&mut input) {
        Ok(_) => print!("{}", base64url_decode_str(&input)),
        Err(err) => print!("ERROR: {:?}", err),
    };
}

pub fn base64url_encode_str(input: &str) -> String {
    encode_config(input, URL_SAFE_NO_PAD)
}

pub fn base64url_encode_filter() {
    let mut input = String::new();
    match stdin().read_to_string(&mut input) {
        Ok(_) => print!("{}", base64url_encode_str(&input)),
        Err(err) => print!("ERROR: {:?}", err),
    };
}

pub fn json_prettyprint_str(input: &str) -> String {
    let sentinel = json!("INVALID INPUT");
    let sentinel_clone = sentinel.clone();
    let error_return = input.to_string();
    let json: Value = from_str(input).unwrap_or(sentinel);
    if json.eq(&sentinel_clone) {
        error_return
    } else {
        serde_json::to_string_pretty(&json).unwrap_or(error_return)
    }
}

pub fn json_prettyprint_filter() {
    let mut input = String::new();
    match stdin().read_to_string(&mut input) {
        Ok(_) => print!("{}", json_prettyprint_str(&input)),
        Err(err) => print!("ERROR: {:?}", err),
    };
}

#[cfg(test)]
mod tests {
    use super::json_prettyprint_str;
    use super::{base64url_decode_str, base64url_encode_str};

    #[test]
    fn test_no_pad() {
        assert_eq!(base64url_encode_str("00"), "MDA");
        assert_eq!("00", base64url_decode_str("MDA"));
    }

    #[test]
    fn test_url_chars() {
        assert_eq!(base64url_encode_str("\u{08}\u{00}~"), "CAB-");
        assert_eq!(base64url_decode_str("CAB-"), "\u{08}\u{00}~");
        assert_eq!(base64url_encode_str("\u{08}\u{00}?"), "CAA_");
        assert_eq!(base64url_decode_str("CAA_"), "\u{08}\u{00}?");
    }

    #[test]
    fn test_invalid_json() {
        assert_eq!(json_prettyprint_str("gar bage"), "gar bage");
    }

    #[test]
    fn test_valid_json() {
        assert_eq!(
            json_prettyprint_str(r#"{"x": 2, "b": 1}"#),
            "{\n  \"b\": 1,\n  \"x\": 2\n}"
        );
    }
}
