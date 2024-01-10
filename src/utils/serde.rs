// Copied from starknet_api

#[derive(Clone, Debug)]
pub enum InnerDeserializationError {
    /// Error parsing the hex string.
    FromHex(hex::FromHexError),
    /// Missing 0x prefix in the hex string.
    MissingPrefix { hex_str: String },
    /// Unexpected input byte count.
    BadInput { expected_byte_count: usize, string_found: String },
}

impl core::convert::From<hex::FromHexError> for InnerDeserializationError {
    fn from(source: hex::FromHexError) -> Self {
        InnerDeserializationError::FromHex(source)
    }
}

pub fn bytes_from_hex_str<const N: usize, const PREFIXED: bool>(
    hex_str: &str,
) -> Result<[u8; N], InnerDeserializationError> {
    let hex_str = if PREFIXED {
        hex_str.strip_prefix("0x").ok_or(InnerDeserializationError::MissingPrefix { hex_str: hex_str.to_string() })?
    } else {
        hex_str
    };

    // Make sure string is not too long.
    if hex_str.len() > 2 * N {
        let mut err_str = "0x".to_owned();
        err_str.push_str(hex_str);
        return Err(InnerDeserializationError::BadInput { expected_byte_count: N, string_found: err_str });
    }

    // Pad if needed.
    let to_add = 2 * N - hex_str.len();
    let padded_str = vec!["0"; to_add].join("") + hex_str;

    Ok(hex::decode(padded_str)?.try_into().expect("Unexpected length of deserialized hex bytes."))
}
