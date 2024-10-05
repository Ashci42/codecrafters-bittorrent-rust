use std::num::ParseIntError;

pub type DecodeResult = Result<serde_json::Value, DecodeError>;

#[derive(Debug)]
pub struct DecodeError;

impl From<ParseIntError> for DecodeError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

pub fn decode(value: &str) -> DecodeResult {
    let mut chars = value.chars();
    let first_char = chars.next();

    match first_char {
        Some(first_char) if first_char.is_ascii_digit() => decode_string(value),
        Some('i') => decode_integer(value),
        _ => Err(DecodeError),
    }
}

fn decode_string(value: &str) -> DecodeResult {
    let mut part_iter = value.splitn(2, ':');

    let s_length = part_iter.next().ok_or(DecodeError)?;
    let s_length: usize = s_length.parse()?;

    let s = part_iter.next().ok_or(DecodeError)?;
    let s = &s[..s_length];

    Ok(serde_json::Value::String(s.into()))
}

fn decode_integer(value: &str) -> DecodeResult {
    let e_index = value.find('e').ok_or(DecodeError)?;
    let integer = &value[1..e_index];
    let integer: i32 = integer.parse()?;

    Ok(serde_json::Value::Number(integer.into()))
}
