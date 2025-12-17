/// The 67-character alphabet used for base67 encoding.
/// Contains uppercase letters (A-Z), lowercase letters (a-z), digits (0-9),
/// and special characters (+, /, |, \, -).
pub const ALPHABET: &[u8; 67] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/|\\-";
