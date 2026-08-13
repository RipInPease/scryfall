#[cfg(test)]
mod test;

use crate::{Deserialize, DesValue, ParseError};

/// An Error object represents a failure to find information 
/// or understand the input you provided to the API
#[derive(Debug, PartialEq)]
pub struct Error {
    /// An integer HTTP status code for this error
    pub status     : i32,

    /// A computer-friendly string representing the appropriate HTTP status code
    pub code       : String,

    /// A human-readable string explaining the error
    pub details    : String,

    /// A computer-friendly string that provides 
    /// additional context for the main error. 
    /// For example, an endpoint many generate HTTP 404 errors 
    /// for different kinds of input. This field will 
    /// provide a label for the specific kind of 404 failure, 
    /// such as ambiguous
    pub error_type : Option<String>,

    /// If your input also generated non-failure warnings, 
    /// they will be provided as human-readable strings in this array
    pub warnings   : Option<Box<[String]>>
}

impl Deserialize for Error {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        let fields = tokens.object_or(ParseError::MismatchedType)?;

        let mut status     : Option<i32>         = None;
        let mut code       : Option<String>      = None;
        let mut details    : Option<String>      = None;
        let mut error_type : Option<String>      = None;
        let mut warnings   : Option<Box<[String]>> = None;

        for (field, val) in fields {
            match &field[..] {
                "object" => {
                    let s = val.string_or(ParseError::MismatchedType)?;
                    if &s != "error" {
                        return Err(ParseError::UnkownVal(s));
                    }
                },
                "status" => {
                    if status.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    status = Some(n)
                },
                "code" => {
                    if code.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    code = Some(val.string_or(ParseError::MismatchedType)?);
                },
                "details" => {
                    if details.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    details = Some(val.string_or(ParseError::MismatchedType)?);
                },
                "error_type" => {
                    if error_type.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    error_type = Some(val.string_or(ParseError::MismatchedType)?);
                },
                "warnings" => {
                    if warnings.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::with_capacity(arr.len());

                    for v in arr {
                        values.push(v.string_or(ParseError::MismatchedType)?);
                    }
                    warnings = Some(values.into_boxed_slice());
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Ok(Self {
            status: status.ok_or(ParseError::ValueExpected)?,
            code: code.ok_or(ParseError::ValueExpected)?,
            details: details.ok_or(ParseError::ValueExpected)?,
            error_type,
            warnings
        })
    }
}