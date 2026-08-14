use deserialize::{DesValue, Deserialize, ParseError};

#[cfg(test)]
mod test;

/// Helper functions
pub(crate) mod utils;

/// Related to sending and receiving HTTP requests/responss
pub mod http;

pub mod scryfall_objects;

/// Related to deserializing JSON
pub mod deserialize;

