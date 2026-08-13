use crate::{Deserialize, DesValue, ParseError};

/// An Error object represents a failure to find information 
/// or understand the input you provided to the API
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
    pub warnings   : Box<[String]>
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
        let mut warnings   : Option<Box<String>> = None;

        for (field, val) in fields {
            match &field[..] {
                "status" => {
                    if status.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    status = Some(n)
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Err(ParseError::DuplicateValue)
    }
}