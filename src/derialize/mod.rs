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
    if &s[0..1] == "{" {
        return Err(ParseError::ExpectedToken('{'));
    }
    
    let mut fields: Vec<(String, DesValue)> = Vec::new();
    return Err(ParseError::ExpectedToken('{'));
}

/// Reads a Field, Value pair
fn read_field_name(s: &str, i: &mut usize) -> Result<String, ParseError> {
    while &s[*i..*i+1] != "\"" {
        *i += 1;

        if *i >= s.len()  {
            return Err(ParseError::ExpectedToken('\"'));
        }
    }
    *i += 1;

    let field_name = read_string_in_quotes(s, i)?;
    if *i >= s.len() || &s[*i..*i+1] != "\"" {
        return Err(ParseError::ExpectedToken('\"'));
    }

    *i += 1;
    Ok(field_name)
}

// Assumes you are already inside quotes
fn read_string_in_quotes(s: &str, i: &mut usize) -> Result<String, ParseError> {
    let mut end = *i;

    while &s[end..end+1] != "\"" {
        end += 1;
        
        if end >= s.len()  {
            return Err(ParseError::ExpectedToken('\"'));
        }
    }

    let res: String = s[*i..end].to_string();
    *i = end;
    Ok(res)
}

/// Reads a numerical string not enclosed in quotes
fn read_string_num(s: &str, i: &mut usize) -> Result<String, ParseError> {
    Err(ParseError::ExpectedToken(','))
}
