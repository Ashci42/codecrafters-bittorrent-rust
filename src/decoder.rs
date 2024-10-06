use std::num::ParseIntError;

pub type DecodeResult = Result<serde_json::Value, DecodeError>;

#[derive(Debug)]
pub struct DecodeError;

impl From<ParseIntError> for DecodeError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

pub struct Decoder<'v> {
    value: &'v str,
}

impl<'v> Decoder<'v> {
    pub fn new(value: &'v str) -> Self {
        Self { value }
    }

    pub fn decode(&mut self) -> DecodeResult {
        let first_char = self.first_char();

        match first_char {
            Some(first_char) if first_char.is_ascii_digit() => self.decode_string(),
            Some('i') => self.decode_integer(),
            Some('l') => self.decode_list(),
            _ => Err(DecodeError),
        }
    }

    fn decode_string(&mut self) -> DecodeResult {
        let (s_length, remainder) = self.value.split_once(':').ok_or(DecodeError)?;
        let s_length: usize = s_length.parse()?;
        let string = &remainder[..s_length];

        self.value = &remainder[s_length..];

        Ok(serde_json::Value::String(string.into()))
    }

    fn decode_integer(&mut self) -> DecodeResult {
        let e_index = self.value.find('e').ok_or(DecodeError)?;
        let integer = &self.value[1..e_index];
        let integer: isize = integer.parse()?;

        self.value = &self.value[e_index + 1..];

        Ok(serde_json::Value::Number(integer.into()))
    }

    fn decode_list(&mut self) -> DecodeResult {
        self.value = &self.value[1..];
        let mut list: Vec<serde_json::Value> = vec![];
        let mut first_char = self.first_char().ok_or(DecodeError)?;
        while first_char != 'e' {
            let json_value = self.decode()?;
            list.push(json_value);
            first_char = self.first_char().ok_or(DecodeError)?;
        }
        self.value = &self.value[1..];

        Ok(serde_json::Value::Array(list))
    }

    fn first_char(&self) -> Option<char> {
        self.value.chars().next()
    }
}
