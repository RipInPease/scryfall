use deserialize::{DesValue, Deserialize, ParseError};

#[cfg(test)]
mod test;

/// Helper functions
pub(crate) mod utils;

/// Related to sending and receiving HTTP requests/responss
pub mod http;

pub mod scryfall_objects;
pub use scryfall_objects::Card;
pub use scryfall_objects::UUID;
pub use scryfall_objects::URI;

/// Related to deserializing JSON
pub mod deserialize;

/// Represents a sequence of other [`ScryfallObject`]
pub struct List {
    pub has_more    : bool,
    pub data        : Vec<ScryfallObject>,
    pub next_page   : Option<URI>,

    /// If this is a list of card object this field will 
    /// be [`Some`]
    pub total_cards : Option<i32>,

    /// Human readable warning returned to your request
    pub warnings    : Option<String>
}

/// All the objects scryfall has to offer
pub enum ScryfallObject {
    Card(Card),
    List(List)
}


