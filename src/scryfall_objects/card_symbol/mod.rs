#[cfg(test)]
mod test;

use crate::{DesValue, Deserialize, ParseError};
use crate::scryfall_objects::*;
use super::card::Color;

#[derive(Debug, PartialEq)]
pub struct CardSymbol {
    /// The plaintext symbol. Often surrounded with curly braces {}. 
    /// Note that not all symbols are ASCII text (for example, {∞})
    pub symbol                  : String,

    /// An alternate version of this symbol, 
    /// if it is possible to write it without curly braces
    pub loose_variant           : Option<String>,

    /// An English snippet that describes this symbol. 
    /// Appropriate for use in alt text or other accessible communication formats
    pub english                 : String,

    /// True if it is possible to write this symbol “backwards”. 
    /// For example, the official symbol {U/P} 
    /// is sometimes written as {P/U} or {P\U} in informal settings. 
    /// Note that the Scryfall API never writes symbols backwards in other responses. 
    /// This field is provided for informational purposes.
    pub transposable            : bool,

    /// True if this is a mana symbol
    pub represents_mana         : bool,

    /// A decimal number representing this symbol’s mana value 
    /// (also knowns as the converted mana cost). 
    /// Note that mana symbols from funny sets can have fractional mana values
    pub mana_value              : Option<f32>,

    /// Should be the same as [`Self::mana_value`]
    /// But Scryfall is dogshit and sends the same shit twice
    pub cmc                     : Option<f32>,

    /// True if this symbol appears in a mana cost on any Magic card. 
    /// For example {20} has this field set to false 
    /// because {20} only appears in Oracle text, not mana costs
    pub appears_in_mana_costs   : bool,

    /// True if this symbol is only used on funny cards or Un-cards
    pub funny                   : bool,

    /// An array of colors that this symbol represents
    pub colors                  : Box<[Color]>,

    /// True if the symbol is a hybrid mana symbol. 
    /// Note that monocolor Phyrexian symbols aren’t considered hybrid
    pub hybrid                  : bool,

    /// True if the symbol is a Phyrexian mana symbol, i.e. it can be paid with 2 life
    pub phyrexian               : bool,

    /// An array of plaintext versions of this symbol that 
    /// Gatherer uses on old cards to describe original printed text. 
    /// For example: {W} has ["oW", "ooW"] as alternates
    pub gatherer_alternates     : Option<Box<[String]>>,

    /// A URI to an SVG image of this symbol on Scryfall’s CDNs
    pub svg_uri                 : Option<URI>
}

impl Deserialize for CardSymbol {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        let fields = tokens.object_or(ParseError::MismatchedType)?;

        let mut symbol                : Option<String>            = None;
        let mut loose_variant         : Option<Option<String>>    = None;
        let mut english               : Option<String>            = None;
        let mut transposable          : Option<bool>              = None;
        let mut represents_mana       : Option<bool>              = None;
        let mut mana_value            : Option<Option<f32>>       = None;
        let mut cmc                   : Option<Option<f32>>       = None;
        let mut appears_in_mana_costs : Option<bool>              = None;
        let mut funny                 : Option<bool>              = None;
        let mut colors                : Option<Box<[Color]>>      = None;
        let mut hybrid                : Option<bool>              = None;
        let mut phyrexian             : Option<bool>              = None;
        let mut gatherer_alternates   : Option<Option<Box<[String]>>> = None;
        let mut svg_uri               : Option<Option<URI>>       = None;

        for (field, val) in fields {
            match &field[..] {
                "object" => {
                    let s = val.string_or(ParseError::MismatchedType)?;
                    if &s != "card_symbol" {
                        return Err(ParseError::UnkownVal(s));
                    }
                },
                "symbol" => {
                    if symbol.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    symbol = Some(val.string_or(ParseError::MismatchedType)?);
                },
                "loose_variant" => {
                    if loose_variant.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    if val.is_null() {
                        loose_variant = Some(None);
                    } else {
                        loose_variant = Some(Some(val.string_or(ParseError::MismatchedType)?));
                    }
                },
                "english" => {
                    if english.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    english = Some(val.string_or(ParseError::MismatchedType)?);
                },
                "transposable" => {
                    if transposable.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    transposable = Some(val.bool_or(ParseError::MismatchedType)?);
                },
                "represents_mana" => {
                    if represents_mana.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    represents_mana = Some(val.bool_or(ParseError::MismatchedType)?);
                },
                "mana_value" => {
                    if val.is_null() {
                        mana_value = Some(None);
                    } else {
                        let n = val.num_or(ParseError::MismatchedType)?;
                        let n = n.parse::<f32>().or(Err(ParseError::UnkownVal(n)))?;
                        mana_value = Some(Some(n));
                    }
                },
                "cmc" => {
                    if val.is_null() {
                        cmc = Some(None);
                    } else {
                        let n = val.num_or(ParseError::MismatchedType)?;
                        let n = n.parse::<f32>().or(Err(ParseError::UnkownVal(n)))?;
                        cmc = Some(Some(n));
                    }
                },
                "appears_in_mana_costs" => {
                    if appears_in_mana_costs.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    appears_in_mana_costs = Some(val.bool_or(ParseError::MismatchedType)?);
                },
                "funny" => {
                    if funny.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    funny = Some(val.bool_or(ParseError::MismatchedType)?);
                },
                "colors" => {
                    if colors.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::with_capacity(arr.len());
                    for v in arr {
                        values.push(Color::deserialize(v)?);
                    }
                    colors = Some(values.into_boxed_slice());
                },
                "hybrid" => {
                    if hybrid.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    hybrid = Some(val.bool_or(ParseError::MismatchedType)?);
                },
                "phyrexian" => {
                    if phyrexian.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    phyrexian = Some(val.bool_or(ParseError::MismatchedType)?);
                },
                "gatherer_alternates" => {
                    if gatherer_alternates.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    if val.is_null() {
                        gatherer_alternates = Some(None);
                    } else {
                        let arr = val.array_or(ParseError::MismatchedType)?;
                        let mut values = Vec::with_capacity(arr.len());
                        for v in arr {
                            values.push(v.string_or(ParseError::MismatchedType)?);
                        }
                        gatherer_alternates = Some(Some(values.into_boxed_slice()));
                    }
                },
                "svg_uri" => {
                    if svg_uri.is_some() {
                        return Err(ParseError::DuplicateValue);
                    }
                    if val.is_null() {
                        svg_uri = Some(None);
                    } else {
                        svg_uri = Some(Some(URI::deserialize(val)?));
                    }
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Ok(Self {
            symbol:                symbol.ok_or(ParseError::ValueExpected)?,
            loose_variant:         loose_variant.unwrap_or(None),
            english:               english.ok_or(ParseError::ValueExpected)?,
            transposable:          transposable.ok_or(ParseError::ValueExpected)?,
            represents_mana:       represents_mana.ok_or(ParseError::ValueExpected)?,
            cmc:                   cmc.unwrap_or(None),
            mana_value:            mana_value.unwrap_or(None),
            appears_in_mana_costs: appears_in_mana_costs.ok_or(ParseError::ValueExpected)?,
            funny:                 funny.ok_or(ParseError::ValueExpected)?,
            colors:                colors.ok_or(ParseError::ValueExpected)?,
            hybrid:                hybrid.ok_or(ParseError::ValueExpected)?,
            phyrexian:             phyrexian.ok_or(ParseError::ValueExpected)?,
            gatherer_alternates:   gatherer_alternates.unwrap_or(None),
            svg_uri:               svg_uri.unwrap_or(None),
        })
    }
}
