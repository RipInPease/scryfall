#[cfg(test)]
mod test;

/// Deserialized value from JSON
#[derive(Debug, PartialEq)]
pub enum DesValue {
    String(String),
    Num(String),
    Bool(bool),

    /// Stored in Name, Value pairs
    Object(Box<[(String, DesValue)]>),
    Array(Box<[DesValue]>),

    /// Equivilant to Rust's [`Option::None`]
    Null
}

impl From<String> for DesValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

macro_rules! desvalue_from_num {
    ($t: ty) => {
        impl From<$t> for DesValue {
            fn from(value: $t) -> Self {
                Self::Num(value.to_string())
            }
        }
    };
}

desvalue_from_num!(i8);
desvalue_from_num!(i16);
desvalue_from_num!(i32);
desvalue_from_num!(i64);
desvalue_from_num!(i128);
desvalue_from_num!(u8);
desvalue_from_num!(u16);
desvalue_from_num!(u32);
desvalue_from_num!(u64);
desvalue_from_num!(u128);
desvalue_from_num!(f32);
desvalue_from_num!(f64);

impl From<bool> for DesValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ExpectedToken(char),
    UnexpectedToken(char),

    /// Where we expect a value, but none was found
    ValueExpected,

    /// Where a value is passed, but we don't know what the fuck it is
    UnkownVal(String)
}

/// Will try to parse an object.
/// If there was no error, will return [`DesValue::Object`].
/// If an error was returned will return `(i, error)` 
/// where `i` is the position of the error and `error` is the error
pub fn parse_json_string(s: String) -> Result<DesValue, (usize, ParseError)> {
    let mut i = 0;
    let res = read_object(&s, &mut i);
    if res.is_err() {
        return Err((i, res.err().unwrap()))
    } else {
        Ok(res.unwrap())
    }
}

/// Will try to read an object.
/// If there was no error, will return [`DesValue::Object`]
fn read_object(s: &str, i: &mut usize) -> Result<DesValue, ParseError> {
    if first_none_whitespace(s, i) != Some('{') {
        return Err(ParseError::ExpectedToken('{'))
    }

    let mut v: Vec<(String, DesValue)> = vec![];

    let c = first_none_whitespace(s, i);
    if c.is_none() {
        return Err(ParseError::ExpectedToken('}'));
    } else if c == Some('}') {
        let vals = v.into_boxed_slice();
        return Ok(DesValue::Object(vals))
    }

    *i -= c.unwrap().len_utf8();

    let field = read_field(s, i)?;

    v.push(field);

    // Keep reading values seperated by comma until we are done
    while let Some(c) = first_none_whitespace(s, i) {
        if c == '}' {
            let vals = v.into_boxed_slice();
            return Ok(DesValue::Object(vals))
        } else if c == ',' {
            let val = read_field(s, i)?;
            v.push(val);
        } else {
            return Err(ParseError::UnexpectedToken(c))
        }
    }

    Err(ParseError::ExpectedToken('}'))
}

fn first_none_whitespace(s: &str, i: &mut usize) -> Option<char> {
    let mut chars = s[*i..].char_indices();
    while let Some((offset, c)) = chars.next() {
        if !c.is_whitespace() {
            *i += offset + c.len_utf8();
            return Some(c)
        }
    }

    None
}

/// Reads a Name, Value pair
fn read_field(s: &str, i: &mut usize) -> Result<(String, DesValue), ParseError> {
    let name = read_field_name(s, i)?;
    if first_none_whitespace(s, i) != Some(':') {
        return Err(ParseError::ExpectedToken(':'))
    }

    let val = read_field_val(s, i)?;
    Ok((name, val))
}

/// Reads a Field name
fn read_field_name(s: &str, i: &mut usize) -> Result<String, ParseError> {
    if first_none_whitespace(s, i) != Some('\"') {
        return Err(ParseError::ExpectedToken('\"'))
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
}

/// Reads a string not enclosed in quotes. This reads until first whitespace after chars, '}', ',' or ']'
fn read_string_out_quotes(s: &str, i: &mut usize) -> String {
    let mut chars = s[*i..].char_indices();
    let mut first_char = false;
    let mut start = *i;

    loop {
        match chars.next() {
            None => {
                let res = s[start..].to_string();
                *i = start;
                return res
            },

            Some((offset, c)) => {
                if c.is_whitespace() && !first_char {
                    start += c.len_utf8();
                } 
                else if c.is_whitespace() && first_char {
                    let res = s[start..*i+offset].to_string();
                    *i += offset;
                    return res;
                } 
                else if !c.is_whitespace() {
                    first_char = true;
                }

                if c == ',' || c == '}' || c == ']' {
                    let res = s[start..*i+offset].to_string();
                    *i += offset;
                    return res;
                }
            }
        }
    }
}


// Reads a field value
fn read_field_val(s: &str, i: &mut usize) -> Result<DesValue, ParseError> {
    let first_char = first_none_whitespace(s, i);
    if first_char.is_none() {
        return Err(ParseError::ValueExpected)
    }

    let first_char = first_char.unwrap();
    match first_char {
        // The field is a string
        '\"' => {
            let s = read_string_in_quotes(s, i)?;
            return Ok(DesValue::String(s))
        },

        // The field is either "True" or "False"; a bool
        'F' | 'f' | 'T' | 't' => {
            *i -= first_char.len_utf8();
            return read_bool(s, i)
        },

        // An int or float
        c if c.is_numeric() => {
            *i -= first_char.len_utf8();
            return read_num(s, i);
        },

        // Array of values
        '[' => {
            *i -= first_char.len_utf8();
            return read_array(s, i);
        },

        // Object with Name, Value pairs
        '{' => {
            *i -= first_char.len_utf8();
            return read_object(s, i);
        },

        // A Null value
        'N' | 'n' => {
            *i -= first_char.len_utf8();
            return read_null(s, i);
        },

        c => {
            return Err(ParseError::UnexpectedToken(c))
        }
    }
}

/// Reads a boolean field
/// If there was no error, will return [`DesValue::Bool`]
fn read_bool(s: &str, i: &mut usize) -> Result<DesValue, ParseError> {
    let s = read_string_out_quotes(s, i).to_lowercase();
    if &s == "true" {
        return Ok(DesValue::Bool(true))
    } else if &s == "false" {
        return Ok(DesValue::Bool(false))
    } else {
        return Err(ParseError::UnkownVal(s))
    }
}

/// Reads a numerical field
/// If there was no error, will return [`DesValue::Num`]
fn read_num(s: &str, i: &mut usize) -> Result<DesValue, ParseError> {
    let res = read_string_out_quotes(s, i);
    let mut decimal = false;
    for c in res.chars() {
        if !c.is_numeric() {
            if c == '.' && !decimal {
                decimal = true
            } else {
                return Err(ParseError::UnexpectedToken(c));
            }
        }
    }

    Ok(DesValue::Num(res))
}

/// Reads a Null field
/// If there was no error, will return [`DesValue::Null`]
fn read_null(s: &str, i: &mut usize) -> Result<DesValue, ParseError> {
    let s = read_string_out_quotes(s, i).to_lowercase();
    if &s == "null" {
        return Ok(DesValue::Null)
    }  else {
        return Err(ParseError::UnkownVal(s))
    }
}

/// Reads the contents of an array.
/// f there was no error, will return [`DesValue::Array`]
fn read_array(s: &str, i: &mut usize) -> Result<DesValue, ParseError> {
    if first_none_whitespace(s, i) != Some('[') {
        return Err(ParseError::ExpectedToken('['))
    }

    let mut v: Vec<DesValue> = vec![];

    let c = first_none_whitespace(s, i);
    if c.is_none() {
        return Err(ParseError::ExpectedToken(']'));
    } else if c == Some(']') {
        let vals = v.into_boxed_slice();
        return Ok(DesValue::Array(vals))
    }

    *i -= c.unwrap().len_utf8();

    let val = read_field_val(s, i)?;
    v.push(val);

    // Keep reading values seperated by comma until we are done
    while let Some(c) = first_none_whitespace(s, i) {
        if c == ']' {
            let vals = v.into_boxed_slice();
            return Ok(DesValue::Array(vals))
        } else if c == ',' {
            let val = read_field_val(s, i)?;
            v.push(val);
        } else {
            return Err(ParseError::UnexpectedToken(c))
        }
    }

    Err(ParseError::ExpectedToken(']'))
}