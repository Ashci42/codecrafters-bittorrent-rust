use std::num::ParseIntError;

#[derive(Debug)]
pub struct DecodeError;

impl From<ParseIntError> for DecodeError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

pub fn decode(value: &str) -> Result<serde_json::Value, DecodeError> {
    let mut chars = value.chars();
    let first_char = chars.next();

    match first_char {
        Some(first_char) if first_char.is_ascii_digit() => decode_string(value),
        _ => Err(DecodeError),
    }
}

fn decode_string(value: &str) -> Result<serde_json::Value, DecodeError> {
    let mut part_iter = value.splitn(2, ':');

    let s_length = part_iter.next().ok_or(DecodeError)?;
    let s_length: usize = s_length.parse()?;

    let s = part_iter.next().ok_or(DecodeError)?;
    let s = &s[..s_length];

    Ok(serde_json::Value::String(s.to_string()))
}
