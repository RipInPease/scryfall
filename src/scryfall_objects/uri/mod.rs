use crate::{Deserialize, DesValue, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub struct URI(pub String);

impl Deserialize for URI {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        match tokens {
            DesValue::String(s) => Ok(Self(s)),
            _ => Err(ParseError::MismatchedType)
        }
    }
}