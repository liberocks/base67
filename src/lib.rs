//! # Base67 Encoding Library
//!
//! This library provides base67 encoding and decoding functionality, similar to base64
//! but using a 67-character alphabet. Base67 encoding converts binary data into a text
//! representation using 67 printable ASCII characters.
//!
//! The encoding scheme processes input bytes in chunks of 3 bytes (24 bits) and converts
//! them into 4 base67 characters. Padding with '=' is used when the input length is not
//! a multiple of 3.
//!
//! # Examples
//!
//! ```
//! use base67::{encode, decode};
//!
//! let data = b"Hello, World!";
//! let encoded = encode(data);
//! let decoded = decode(&encoded).unwrap();
//! assert_eq!(decoded, data);
//! ```

mod constants;
mod decode;
mod encode;

pub use decode::decode;
pub use encode::encode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(b""), "");
    }

    #[test]
    fn test_decode_empty() {
        assert_eq!(decode("").unwrap(), b"");
    }

    #[test]
    fn test_padding_1() {
        let data = b"M";
        let encoded = encode(data);
        assert!(encoded.ends_with("=="));
        assert_eq!(decode(&encoded).unwrap(), data);
    }

    #[test]
    fn test_padding_2() {
        let data = b"Ma";
        let encoded = encode(data);
        assert!(encoded.ends_with("="));
        assert_eq!(decode(&encoded).unwrap(), data);
    }

    #[test]
    fn test_known_values() {
        assert_eq!(encode(b"foo"), "WVgA");
        assert_eq!(encode(&[0, 0, 0]), "AAAA");
        assert_eq!(encode(b"Rust"), "R|4ABx==");
        assert_eq!(encode(b"Hello world"), "Pz+SXqDwaBsnGMK=");
    }

    #[test]
    fn test_round_trip() {
        let data = b"Hello World!";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_round_trip_padding() {
        for len in 1..=100 {
            let data: Vec<u8> = (0..len).map(|i| (i % 255) as u8).collect();
            let encoded = encode(&data);
            let decoded = decode(&encoded).expect("Failed to decode");
            assert_eq!(data, decoded, "Failed round trip for length {}", len);
        }
    }
}
