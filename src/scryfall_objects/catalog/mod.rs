#[cfg(test)]
mod test;

use crate::scryfall_objects::*;
use crate::{DesValue, Deserialize, ParseError};

/// A Catalog object contains an array of Magic datapoints 
/// (words, card values, etc). Catalog objects are provided 
/// by the API as aids for building other Magic software and 
/// understanding possible values for a field on Card objects
#[derive(Debug, PartialEq)]
pub struct Catalog {
    /// A link to the current catalog on Scryfall’s API
    uri             : URI,

    /// The number of items in the data array.
    total_values    : i32,

    /// An array of datapoints, as strings
    data            : Box<[String]>
}

impl Deserialize for Catalog {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        let fields = tokens.object_or(ParseError::MismatchedType)?;

        let mut uri             : Option<URI>           = None;
        let mut total_values    : Option<i32>           = None;
        let mut data            : Option<Box<[String]>> = None;

        for (field, val) in fields {
            match &field[..] {
                "object" => {
                    let s = val.string_or(ParseError::MismatchedType)?;
                    if &s != "catalog" {
                        return Err(ParseError::UnkownVal(s));
                    }
                },
                "uri" => {
                    if uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let v = URI::deserialize(val)?;
                    uri = Some(v)
                },
                "total_values" => {
                    if total_values.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().map_err(|_| ParseError::UnkownVal(n))?;
                    total_values = Some(n)
                },
                "data" => {
                    if data.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::with_capacity(arr.len());

                    for v in arr {
                        values.push(v.string_or(ParseError::MismatchedType)?);
                    }

                    data = Some(values.into_boxed_slice())
                },
                _ => {
                    return Err(ParseError::UnkownVal(field));
                }
            }
        }

        Ok(Self {
            uri: uri.ok_or(ParseError::ValueExpected)?,
            total_values: total_values.ok_or(ParseError::ValueExpected)?,
            data: data.ok_or(ParseError::ValueExpected)?,
        })
    }
}