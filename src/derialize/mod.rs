/// Deserialized value from JSON
pub enum DesValue {
    String(String),
    Num(String),
    Bool(bool),

    /// Stored in Field, Value pairs
    Object(Box<[(String, DesValue)]>),
    Array(Box<[DesValue]>)
}

pub enum ParseError {
    ExpectedToken(char),
}

/// Will try to parse an object.
/// If there was no error, will return [`DesValue::Object`]
pub fn parse_string(s: String) -> Result<DesValue, ParseError> {
    parse_inner(&s)
}

/// Will try to parse an object.
/// If there was no error, will return [`DesValue::Object`]
fn parse_inner(s: &str) -> Result<DesValue, ParseError> {
    let mut curly_opened = false;
    let mut i = 0;

    if !(&s[0..1] == "{") {
        return Err(ParseError::ExpectedToken('{'));
    } else {
        curly_opened = true;
    }


    Err(ParseError::ExpectedToken('{'))
}