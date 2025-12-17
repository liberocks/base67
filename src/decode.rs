use crate::constants::ALPHABET;

/// Decodes a base67-encoded string back into binary data.
///
/// This function converts a base67-encoded string back into the original binary data.
/// The input string must have a length that is a multiple of 4, as each group of 4
/// base67 characters represents 3 bytes of data (or fewer with padding).
///
/// # Arguments
///
/// * `input` - A string slice containing the base67-encoded data to decode.
///
/// # Returns
///
/// * `Result<Vec<u8>, String>` - On success, returns a vector containing the decoded
///   binary data. On failure, returns an error message describing the problem.
///
/// # Errors
///
/// This function will return an error if:
/// * The input length is not a multiple of 4
/// * The input contains invalid characters not in the base67 alphabet
///
/// # Examples
///
/// ```
/// use base67::decode;
///
/// // Decode a base67 string
/// let decoded = decode("WVgA").unwrap();
/// assert_eq!(decoded, b"foo");
///
/// // Decode empty input
/// assert_eq!(decode("").unwrap(), b"");
///
/// // Handle invalid input
/// assert!(decode("ABC").is_err()); // Invalid length
/// ```
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    if !input.len().is_multiple_of(4) {
        return Err("Invalid input length".to_string());
    }

    let input_bytes = input.as_bytes();
    let mut output = Vec::with_capacity(input.len() * 3 / 4);

    for chunk in input_bytes.chunks(4) {
        let c0 = chunk[0];
        let c1 = chunk[1];
        let c2 = chunk[2];
        let c3 = chunk[3];

        if c2 == b'=' && c3 == b'=' {
            let idx0 = decode_char(c0)?;
            let idx1 = decode_char(c1)?;
            let val = (idx0 as u32) * 67 + (idx1 as u32);
            output.push((val & 0xFF) as u8);
        } else if c3 == b'=' {
            let idx0 = decode_char(c0)?;
            let idx1 = decode_char(c1)?;
            let idx2 = decode_char(c2)?;
            let val = (idx0 as u32) * 67u32.pow(2) + (idx1 as u32) * 67 + (idx2 as u32);
            output.push(((val >> 8) & 0xFF) as u8);
            output.push((val & 0xFF) as u8);
        } else {
            let idx0 = decode_char(c0)?;
            let idx1 = decode_char(c1)?;
            let idx2 = decode_char(c2)?;
            let idx3 = decode_char(c3)?;

            let val = (idx0 as u32) * 67u32.pow(3)
                + (idx1 as u32) * 67u32.pow(2)
                + (idx2 as u32) * 67
                + (idx3 as u32);

            output.push(((val >> 16) & 0xFF) as u8);
            output.push(((val >> 8) & 0xFF) as u8);
            output.push((val & 0xFF) as u8);
        }
    }

    Ok(output)
}

/// Decodes a single base67 character into its numeric index.
///
/// This is a helper function that converts a base67 character back into its
/// corresponding index (0-66) in the ALPHABET array.
///
/// # Arguments
///
/// * `c` - A byte representing a single base67 character.
///
/// # Returns
///
/// * `Result<u8, String>` - On success, returns the index (0-66) of the character
///   in the alphabet. On failure, returns an error message indicating the invalid character.
///
/// # Errors
///
/// This function will return an error if the character is not found in the base67 alphabet.
fn decode_char(c: u8) -> Result<u8, String> {
    ALPHABET
        .iter()
        .position(|&x| x == c)
        .map(|i| i as u8)
        .ok_or_else(|| format!("Invalid character: {}", c as char))
}
