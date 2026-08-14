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
