#[cfg(test)]
mod test;

use crate::scryfall_objects::*;
use crate::{DesValue, Deserialize, ParseError};

/// Rulings represent Oracle rulings, Wizards of the Coast set release notes, 
/// or Scryfall notes for a particular card
#[derive(Debug, PartialEq)]
pub struct Ruling {
    /// The Oracle ID of the card this ruling is associated with
    pub oracle_id    : UUID,
    /// A computer-readable string indicating which company produced this ruling, either wotc or scryfall
    pub source       : String,

    /// The date when the ruling or note was published
    pub published_at : String,

    /// The text of the ruling
    pub comment      : String,
}

impl Deserialize for Ruling {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        let fields = tokens.object_or(ParseError::MismatchedType)?;

        let mut oracle_id : Option<UUID>   = None;
        let mut source    : Option<String> = None;
        let mut published_at      : Option<String> = None;
        let mut comment   : Option<String> = None;

        for (field, val) in fields {
            match &field[..] {
                "object" => {
                    let s = val.string_or(ParseError::MismatchedType)?;
                    if &s != "ruling" {
                        return Err(ParseError::UnkownVal(s));
                    }
                },
                "oracle_id" => {
                    if oracle_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    oracle_id = Some(UUID::deserialize(val)?);
                },
                "source" => {
                    if source.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    source = Some(val.string_or(ParseError::MismatchedType)?);
                },
                "published_at" => {
                    if published_at.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    published_at = Some(val.string_or(ParseError::MismatchedType)?);
                },
                "comment" => {
                    if comment.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    comment = Some(val.string_or(ParseError::MismatchedType)?);
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Ok(Self {
            oracle_id: oracle_id.ok_or(ParseError::ValueExpected)?,
            source: source.ok_or(ParseError::ValueExpected)?,
            published_at: published_at.ok_or(ParseError::ValueExpected)?,
            comment: comment.ok_or(ParseError::ValueExpected)?,
        })
    }
}
