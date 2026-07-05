#[cfg(test)]
mod test;

pub mod card;

/// Derialize all types here
pub(crate) mod derialize;


use std::io::{Error as IOError, ErrorKind, Read};

/// Only JSON becuase fuck you
pub trait Derialize {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DerializeError>
    where Self: Sized;
}

pub enum DerializeError {
    IOError(IOError),
    /// Unexpected end of
    UEO,
    /// Name of the unknown field
    UnknownField(String),
    /// Name of the field that is missing
    MissingField(String),
    ExpectedToken(char),
}

impl From<IOError> for DerializeError {
    fn from(value: IOError) -> Self {
        if value.kind() == ErrorKind::UnexpectedEof {
            Self::UEO
        } else {
            Self::IOError(value)
        }
    }
}

pub struct URI;

#[derive(Debug, Clone)]
pub struct UUID(pub [u8;16]);

impl UUID {
    /// Turns the ascii 1-A to numerical 1-16
    fn ascii_to_hex(val: u8) -> Result<u8, ()> {
        println!("val: {}", val);
        // ascii value for digits 0-9
        if val >= 48 && val <= 57 {
            Ok(val - 48)
        } 
        // ascii value for digits A-F
        else if val >= 65 && val <= 70 {
            Ok(val - 55)
        }
        // ascii value for digits a-f
        else if val >= 97 && val <= 102 {
            Ok(val - 87)
        } else {
            Err(())
        }
    }
}

/// Where this is an array of 36 chars
impl TryFrom<[u8; 36]> for UUID {
    type Error = ();

    fn try_from(value: [u8; 36]) -> Result<Self, Self::Error> {
        let mut bytes = [0;16];
        let mut i = 0;
        let mut j = 0;

        // Read until the first dash
        while j < 8 {
            let d1 = UUID::ascii_to_hex(value[j])?;
            let d2 = UUID::ascii_to_hex(value[j+1])?;

            let b = d1 * 16 + d2;
            bytes[i] = b;
            i += 1;
            j += 2;
        }
        j += 1;

        // Read until the second dash
        while j < 13 {
            let d1 = UUID::ascii_to_hex(value[j])?;
            let d2 = UUID::ascii_to_hex(value[j+1])?;

            let b = d1 * 16 + d2;
            bytes[i] = b;
            i += 1;
            j += 2;
        }
        j += 1;

        // Read until the third dash
        while j < 18 {
            let d1 = UUID::ascii_to_hex(value[j])?;
            let d2 = UUID::ascii_to_hex(value[j+1])?;

            let b = d1 * 16 + d2;
            bytes[i] = b;
            i += 1;
            j += 2;
        }

        // Read until the fourth dash
        while j < 23 {
            let d1 = UUID::ascii_to_hex(value[j])?;
            let d2 = UUID::ascii_to_hex(value[j+1])?;

            let b = d1 * 16 + d2;
            bytes[i] = b;
            i += 1;
            j += 2;
        }
        j += 1;

        // Read until the end
        while j < 35 {
            let d1 = UUID::ascii_to_hex(value[j])?;
            let d2 = UUID::ascii_to_hex(value[j+1])?;

            let b = d1 * 16 + d2;
            bytes[i] = b;
            i += 1;
            j += 2;
        }

        Ok(Self(bytes))
    }
}


pub struct List {
    pub has_more    : bool,
    pub data        : Vec<Data>,
    pub next_page   : Option<URI>,

    /// If this is a list of card object this field will 
    /// be [`Some`]
    pub total_cards : Option<i32>,

    /// Human readable warning returned to your request
    pub warnings    : Option<String>
}

/// The different kinds of data a [`List`] can contain
pub enum Data {
    Cards(card::Card),
}
