#[cfg(test)]
mod test;

use super::*;
use crate::deserialize::{Deserialize, DesValue, ParseError};

/// Represents a sequence of other [`ScryfallObject`]
pub struct List {
    pub data        : Box<[ScryfallObject]>,
    pub has_more    : bool,
    pub next_page   : Option<URI>,

    /// If this is a list of card object this field will 
    /// be [`Some`]
    pub total_cards : Option<i32>,

    /// Human readable warning returned to your request
    pub warnings    : Option<Box<[String]>>
}

impl List {
    /// Reads a single entry in `Self.Data`
    fn parse_data_entry(tokens: DesValue) -> Result<ScryfallObject, ParseError> {
        if let DesValue::Object(fields) = &tokens {
            match Self::inner_object(fields)? {
                "card" => {
                    let res = Card::deserialize(tokens);
                    return Ok(ScryfallObject::Card(res?))
                },
                "error" => return Ok(ScryfallObject::Error(Error::deserialize(tokens)?)),
                "ruling" => return Ok(ScryfallObject::Ruling(Ruling::deserialize(tokens)?)),
                "catalog" => return Ok(ScryfallObject::Catalog(Catalog::deserialize(tokens)?)),
                "card_symbol" => return Ok(ScryfallObject::CardSymbol(CardSymbol::deserialize(tokens)?)),
                s => return Err(ParseError::UnkownVal(s.to_string()))
            }
        } else {
            return Err(ParseError::MismatchedType)
        }
    }

    /// Gives the object a [`DesValue`] contains.
    /// 
    /// Returns [`ParseError::ValueExpected`] if the inner object did
    /// contain an "object field"
    /// or [ParseError::MismatchedType] if the value of the "object"
    /// was not of type [`DesValue::String`]
    fn inner_object<'a>(fields: &'a [(String, DesValue)]) -> Result<&'a str, ParseError> {
        for (field, val) in fields {
            if &field[..] == "object" {
                if let DesValue::String(s) = val {
                    return Ok(s)
                } else {
                    return Err(ParseError::MismatchedType)
                }
            }
        }

        Err(ParseError::ValueExpected)
    }
}

impl Deserialize for List {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        let fields = tokens.object_or(ParseError::MismatchedType)?;

        let mut data: Option<Box<[ScryfallObject]>> = None;
        let mut has_more: Option<bool> = None;
        let mut next_page: Option<URI> = None;
        let mut total_cards: Option<i32> = None;
        let mut warnings: Option<Box<[String]>> = None;

        for (field, val) in fields {
            match &field[..] {
                "object" => {
                    let s = val.string_or(ParseError::MismatchedType)?;
                    if &s != "list" {
                        return Err(ParseError::UnkownVal(s));
                    }
                },
                "data" => {
                    if data.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut vals = Vec::with_capacity(arr.len());

                    for tokens in arr {
                        let entry = Self::parse_data_entry(tokens)?;
                        vals.push(entry);
                    } 

                    data = Some(vals.into_boxed_slice())
                },
                "has_more" => {
                    if has_more.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let v = val.bool_or(ParseError::MismatchedType)?;
                    has_more = Some(v)
                },
                "next_page" => {
                    if next_page.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let v = URI::deserialize(val)?;
                    next_page = Some(v)
                },
                "total_cards" => {
                    if total_cards.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().map_err(|_| ParseError::UnkownVal(n))?;
                    total_cards = Some(n)
                },
                "warnings" => {
                    if warnings.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut vals = Vec::with_capacity(arr.len());

                    for tokens in arr {
                        let entry = tokens.string_or(ParseError::MismatchedType)?;
                        vals.push(entry);
                    } 

                    warnings = Some(vals.into_boxed_slice())
                },
                _ => return Err(ParseError::UnkownVal(field)),
            }
        }


        Ok(Self {
            data: data.ok_or(ParseError::ValueExpected)?,
            has_more: has_more.ok_or(ParseError::ValueExpected)?,
            next_page,
            total_cards,
            warnings
        })
    }
}