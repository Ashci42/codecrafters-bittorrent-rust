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
            Some('d') => self.decode_dictionary(),
            _ => Err(DecodeError),
        }
    }

    fn decode_string(&mut self) -> DecodeResult {
        let string = self.decode_string_raw()?;

        Ok(serde_json::Value::String(string))
    }

    fn decode_integer(&mut self) -> DecodeResult {
        let e_index = self.value.find('e').ok_or(DecodeError)?;
        let integer = &self.value[1..e_index];
        let integer: isize = integer.parse()?;

        self.value = &self.value[e_index + 1..];

        Ok(serde_json::Value::Number(integer.into()))
    }

    fn decode_list(&mut self) -> DecodeResult {
        self.remove_first_char();
        let mut list: Vec<serde_json::Value> = vec![];
        let mut first_char = self.first_char().ok_or(DecodeError)?;
        while first_char != 'e' {
            let value = self.decode()?;
            list.push(value);
            first_char = self.first_char().ok_or(DecodeError)?;
        }
        self.remove_first_char();

        Ok(serde_json::Value::Array(list))
    }

    fn first_char(&self) -> Option<char> {
        self.value.chars().next()
    }

    fn decode_dictionary(&mut self) -> DecodeResult {
        self.remove_first_char();
        let mut dictionary = serde_json::Map::new();
        let mut first_char = self.first_char().ok_or(DecodeError)?;
        while first_char != 'e' {
            let key = self.decode_string_raw()?;
            let value = self.decode()?;
            dictionary.insert(key, value);
            first_char = self.first_char().ok_or(DecodeError)?;
        }
        self.remove_first_char();

        Ok(serde_json::Value::Object(dictionary))
    }

    fn decode_string_raw(&mut self) -> Result<String, DecodeError> {
        let (s_length, remainder) = self.value.split_once(':').ok_or(DecodeError)?;
        let s_length: usize = s_length.parse()?;
        let string = &remainder[..s_length];

        self.value = &remainder[s_length..];

        Ok(string.to_string())
    }

    fn remove_first_char(&mut self) {
        self.value = &self.value[1..];
    }
}
