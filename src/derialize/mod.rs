#[cfg(test)]
mod test;

/// Deserialized value from JSON
pub enum DesValue {
    String(String),
    Num(String),
    Bool(bool),

    /// Stored in Field, Value pairs
    Object(Box<[(String, DesValue)]>),
    Array(Box<[DesValue]>)
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ExpectedToken(char),
    UnexpectedTopen(char)
}

/// Will try to parse an object.
/// If there was no error, will return [`DesValue::Object`]
pub fn parse_string(s: String) -> Result<DesValue, ParseError> {
    let mut i = 0;
    parse_object(&s, &mut i)
}

/// Will try to parse an object.
/// If there was no error, will return [`DesValue::Object`]
fn parse_object(s: &str, i: &mut usize) -> Result<DesValue, ParseError> {
    let mut i = 0;
    let mut chars = s[i..].char_indices();

    Err(ParseError::ExpectedToken('{'))
}

/// Reads a Field name
fn read_field_name(s: &str, i: &mut usize) -> Result<String, ParseError> {
    let mut chars = s[*i..].char_indices();

    // Find the opening quote
    loop {
        match chars.next() {
            None              => return Err(ParseError::ExpectedToken('\"')),

            Some((offset, c)) => {
                if !c.is_whitespace() && c != '\"' {
                    return Err(ParseError::UnexpectedTopen(c))
                } else if c == '\"' {
                    *i += offset + c.len_utf8();
                    break;
                }
            }
        }
    }

    // Read the string inside the quotes
    read_string_in_quotes(s, i)
}

// Assumes you are already inside quotes
fn read_string_in_quotes(s: &str, i: &mut usize) -> Result<String, ParseError> {
    let mut chars = s[*i..].char_indices();

    loop {
        match chars.next() {
            None              => return Err(ParseError::ExpectedToken('\"')),
            Some((offset, c)) => {
                if c == '\"' {
                    let s = s[*i..*i+offset].to_string();
                    *i += offset + c.len_utf8();
                    return Ok(s)
                }
            }
        }
    }


    Err(ParseError::ExpectedToken('\"'))
}

/// Reads a string not enclosed in quotes. This reads until first whitespace after chars, '}' or ','
fn read_string_out_quotes(s: &str, i: &mut usize) -> Result<String, ParseError> {
    let mut chars = s[*i..].char_indices();
    let mut first_char = false;
    let mut start = *i;

    loop {
        match chars.next() {
            None => {
                let res = s[start..].to_string();
                *i = start;
                return Ok(res)
            },

            Some((offset, c)) => {
                if c.is_whitespace() && !first_char {
                    start += c.len_utf8();
                } 
                else if c.is_whitespace() && first_char {
                    let res = s[start..*i+offset].to_string();
                    *i += offset;
                    return Ok(res);
                } 
                else if !c.is_whitespace() {
                    first_char = true;
                }

                if c == ',' || c == '}' {
                    let res = s[start..*i+offset].to_string();
                    *i += offset;
                    return Ok(res);
                }
            }
        }
    }
}
