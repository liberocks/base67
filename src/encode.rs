use crate::constants::ALPHABET;

/// Encodes binary data into a base67 string representation.
///
/// This function converts arbitrary binary data into a text string using the base67
/// encoding scheme. The encoding processes input bytes in chunks of 3 bytes (24 bits)
/// and converts each chunk into 4 base67 characters. If the input length is not a
/// multiple of 3, padding characters ('=') are added to the output.
///
/// # Arguments
///
/// * `input` - A byte slice containing the binary data to encode.
///
/// # Returns
///
/// * `String` - The base67-encoded string representation of the input data.
///
/// # Examples
///
/// ```
/// use base67::encode;
///
/// // Encode a simple string
/// let encoded = encode(b"Hello");
/// assert!(!encoded.is_empty());
///
/// // Encode empty input
/// assert_eq!(encode(b""), "");
///
/// // Encode known values
/// assert_eq!(encode(b"foo"), "WVgA");
/// assert_eq!(encode(&[0, 0, 0]), "AAAA");
/// ```
pub fn encode(input: &[u8]) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut output = String::with_capacity(input.len() * 4 / 3 + 4);

    for chunk in input.chunks(3) {
        match chunk.len() {
            3 => {
                let val = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
                let mut n = val;
                let d3 = n % 67;
                n /= 67;
                let d2 = n % 67;
                n /= 67;
                let d1 = n % 67;
                n /= 67;
                let d0 = n % 67;

                output.push(ALPHABET[d0 as usize] as char);
                output.push(ALPHABET[d1 as usize] as char);
                output.push(ALPHABET[d2 as usize] as char);
                output.push(ALPHABET[d3 as usize] as char);
            }
            2 => {
                let val = ((chunk[0] as u32) << 8) | (chunk[1] as u32);

                let mut n = val;
                let d2 = n % 67;
                n /= 67;
                let d1 = n % 67;
                n /= 67;
                let d0 = n % 67;

                output.push(ALPHABET[d0 as usize] as char);
                output.push(ALPHABET[d1 as usize] as char);
                output.push(ALPHABET[d2 as usize] as char);
                output.push('=');
            }
            1 => {
                let val = chunk[0] as u32;
                let mut n = val;
                let d1 = n % 67;
                n /= 67;
                let d0 = n % 67;

                output.push(ALPHABET[d0 as usize] as char);
                output.push(ALPHABET[d1 as usize] as char);
                output.push('=');
                output.push('=');
            }
            _ => unreachable!(),
        }
    }

    output
}
