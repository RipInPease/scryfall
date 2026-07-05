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
pub struct UUID([u8;16]);


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
