/// The reason why you're here
pub mod card;
pub use card::Card;

/// Universal unique id
mod uuid;
pub use uuid::UUID;

/// Unique resource identifier
mod uri;
pub use uri::URI;

/// Error object
mod error;
pub use error::Error;

/// Rulings for a card
mod ruling;
pub use ruling::Ruling;

/// Illustrated symbol that may appear in card’s mana cost or Oracle text
mod card_symbol;
pub use card_symbol::CardSymbol;

/// Contains an array of Magic datapoints (words, card values, etc)
mod catalog;
pub use catalog::Catalog;

/// Represents a sequence of other [`ScryfallObject`]
mod list;
pub use list::List;


/// All the objects scryfall has to offer
pub enum ScryfallObject {
    List(List),
    Card(Card),
    Error(Error),
    Ruling(Ruling),
    CardSymbol(CardSymbol),
    Catalog(Catalog)
}

impl ScryfallObject {
    /// Returns `true` if this is of the type [`Self::List`]
    pub fn is_list(&self) -> bool {
        match self {
            Self::List(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is of the type [`Self::Card`]
    pub fn is_card(&self) -> bool {
        match self {
            Self::Card(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is of the type [`Self::Error`]
    pub fn is_error(&self) -> bool {
        match self {
            Self::Error(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is of the type [`Self::Ruling`]
    pub fn is_ruling(&self) -> bool {
        match self {
            Self::Ruling(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is of the type [`Self::CardSymbol`]
    pub fn is_card_symbol(&self) -> bool {
        match self {
            Self::CardSymbol(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is of the type [`Self::Catalog`]
    pub fn is_catalog(&self) -> bool {
        match self {
            Self::Catalog(_) => true,
            _ => false,
        }
    }
}
