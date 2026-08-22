#[cfg(test)]
mod test;

mod connection;
pub use connection::{Connection, RestRequest};

mod response;
pub use response::{Response, HttpStatus};

use std::io::Error as IOError;

/// Converts a human readable string query to acceptable http.
/// It does this by being lazy and converting each character to its hex value
/// 
/// # Example
/// 
/// `this is a query` returs `%74%68%69%73%20%69%73%20%61%20%71%75%65%72%79`
pub (crate) fn query_string_to_http(s: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut res = String::with_capacity(s.len() * 3);

    for b in s.bytes() {
        res.push('%');
        res.push(HEX[(b >> 4) as usize] as char);
        res.push(HEX[(b & 0xF) as usize] as char);
    }

    res
}


#[derive(Debug)]
pub enum Error {
    IoError(IOError),

    /// Where a value given is not up to the standard I pretend to follow
    IncorrectValue {
        expected: &'static str,
        got     : String,
    },

    /// If we expected some utf-data but got some other shit
    NonUTF8,

    /// What the fuck did I just read
    ProtocolDeviation
}

impl Error {
    /// Returns `true` if it is equal to [`Error::IoError`]
    pub fn is_io_error(&self) -> bool {
        match self {
            Self::IoError(_) => true,
            _ => false
        }
    }

    /// Returns `true` if it is equal to [`Error::IncorrectValue`]
    pub fn is_incorrect_value(&self) -> bool {
        match self {
            Self::IncorrectValue{..} => true,
            _ => false
        }
    }

    /// Returns `true` if it is equal to [`Error::NonUTF8`]
    pub fn is_non_utf8(&self) -> bool {
        match self {
            Self::NonUTF8 => true,
            _ => false
        }
    }

    /// Returns the contained [`Error::IncorrectValue`], consuming the `self` value
    ///
    /// # Panics
    ///
    /// Panics if the value is not equal [`Error::IncorrectValue`]
    ///
    pub fn unwrap_incorrect_value(self) -> (&'static str, String) {
        match self {
            Self::IncorrectValue { expected, got } => (expected, got),
            _ => panic!("Called unwrap_incorrect_value on non-incorrect-value error"),
        }
    }

    /// Returns the contained [`Error::IoError`], consuming the `self` value
    ///
    /// # Panics
    ///
    /// Panics if the value is not equal [`Error::IoError`]
    ///
    pub fn unwrap_io_error(self) -> IOError {
        match self {
            Self::IoError(e) => e,
            _ => panic!("Called unwrap_io_error on non-io error"),
        }
    }

    /// Returns successfully if the value is [`Error::NonUTF8`], consuming the `self` value
    ///
    /// # Panics
    ///
    /// Panics if the value is not equal [`Error::NonUTF8`]
    ///
    pub fn unwrap_non_utf8(self) {
        match self {
            Self::NonUTF8 => (),
            _ => panic!("Called unwrap_non_utf8 on non-utf8 error"),
        }
    }
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self {
        Self::IoError(value)
    }
}

