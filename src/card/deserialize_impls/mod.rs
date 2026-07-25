#[cfg(test)]
mod test;

use super::*;
use crate::deserialize::{Deserialize, DesValue, ParseError};
use crate::URI;

impl Deserialize for Color {
    fn deserialize(tokens: DesValue) -> Result<Self, deserialize::ParseError>
    where Self: Sized
    {
        if let DesValue::String(s) = tokens {
            match &s[..] {
                "W" => Ok(Self::White),
                "U" => Ok(Self::Blue),
                "B" => Ok(Self::Black),
                "R" => Ok(Self::Red),
                "G" => Ok(Self::Green),
                "C" => Ok(Self::Colorless),
                _   => Err(ParseError::UnkownVal(s))
            }
        } else {
            Err(ParseError::MismatchedType)
        }
    }
}


impl Deserialize for Language {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if let DesValue::String(s) = tokens {
            match &s[..] {
                "en"  => Ok(Self::EN),
                "es"  => Ok(Self::ES),
                "fr"  => Ok(Self::FR),
                "de"  => Ok(Self::DE),
                "it"  => Ok(Self::IT),
                "pt"  => Ok(Self::PT),
                "ja"  => Ok(Self::JA),
                "ko"  => Ok(Self::KO),
                "ru"  => Ok(Self::RU),
                "zhs" => Ok(Self::ZHS),
                "zht" => Ok(Self::ZHT),
                "he"  => Ok(Self::HE),
                "la"  => Ok(Self::LA),
                "grc" => Ok(Self::GRC),
                "ar"  => Ok(Self::AR),
                "sa"  => Ok(Self::SA),
                "ph"  => Ok(Self::PH),
                "qya" => Ok(Self::QYA),
                "dw"  => Ok(Self::DW),
                _     => Err(ParseError::UnkownVal(s))
            }
        } else {
            Err(ParseError::MismatchedType)
        }
    }
}


impl Deserialize for Layout {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if let DesValue::String(s) = tokens {
            match &s[..] {
                "normal"             => Ok(Self::Normal),
                "split"              => Ok(Self::Split),
                "flip"               => Ok(Self::Flip),
                "transform"          => Ok(Self::Transform),
                "modal_dfc"          => Ok(Self::ModalDFC),
                "meld"               => Ok(Self::Meld),
                "leveler"            => Ok(Self::Leveler),
                "class"              => Ok(Self::Class),
                "case"               => Ok(Self::Case),
                "saga"               => Ok(Self::Saga),
                "adventure"          => Ok(Self::Adventure),
                "prepare"            => Ok(Self::Prepare),
                "mutate"             => Ok(Self::Mutate),
                "prototype"          => Ok(Self::Prototype),
                "battle"             => Ok(Self::Battle),
                "planar"             => Ok(Self::Planar),
                "scheme"             => Ok(Self::Scheme),
                "vanguard"           => Ok(Self::Vanguard),
                "token"              => Ok(Self::Token),
                "double_faced_token" => Ok(Self::DoubleFacedToken),
                "emblem"             => Ok(Self::Emblem),
                "augment"            => Ok(Self::Augment),
                "host"               => Ok(Self::Host),
                "art_series"         => Ok(Self::ArtSeries),
                "reversible_card"    => Ok(Self::ReversibleCard),
                _                    => Err(ParseError::UnkownVal(s))
            }
        } else {
            Err(ParseError::MismatchedType)
        }
    }
}


impl Deserialize for ImageURIs {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_object() {
            return Err(ParseError::MismatchedType)
        }

        let fields = tokens.unwrap_object();

        let mut png:         Option<URI> = None;
        let mut small:       Option<URI> = None;
        let mut normal:      Option<URI> = None;
        let mut large:       Option<URI> = None;
        let mut border_crop: Option<URI> = None;
        let mut art_crop:    Option<URI> = None;
        let mut thumb:       Option<URI> = None;
        let mut grid:        Option<URI> = None;
        let mut display:     Option<URI> = None;
        let mut crop:        Option<URI> = None;
        let mut art:         Option<URI> = None;

        for (name, val) in fields {
            match &name[..] {
                "png"         => {
                    let uri = URI::deserialize(val)?;
                    png = Some(uri)
                },
                "small"       => {
                    let uri = URI::deserialize(val)?;
                    small = Some(uri)
                },
                "normal"      => {
                    let uri = URI::deserialize(val)?;
                    normal = Some(uri)
                },
                "large"       => {
                    let uri = URI::deserialize(val)?;
                    large = Some(uri)
                },
                "border_crop" => {
                    let uri = URI::deserialize(val)?;
                    border_crop = Some(uri)
                },
                "art_crop"    => {
                    let uri = URI::deserialize(val)?;
                    art_crop = Some(uri)
                },
                "thumb"       => {
                    let uri = URI::deserialize(val)?;
                    thumb = Some(uri)
                },
                "grid"        => {
                    let uri = URI::deserialize(val)?;
                    grid = Some(uri)
                },
                "display"     => {
                    let uri = URI::deserialize(val)?;
                    display = Some(uri)
                },
                "crop"        => {
                    let uri = URI::deserialize(val)?;
                    crop = Some(uri)
                },
                "art"         => {
                    let uri = URI::deserialize(val)?;
                    art = Some(uri)
                },
                _ => ()
            }
        }

        let res = Self {
            png,
            small,
            normal,
            large,
            border_crop,
            art_crop,
            thumb,
            grid,
            display,
            crop,
            art
        };

        return Ok(res)
    }
}


impl Deserialize for CardFace {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        if !tokens.is_object() {
            return Err(ParseError::MismatchedType);
        }

        let fields = tokens.unwrap_object();

        let mut artist: Option<String> = None;
        let mut artist_id: Option<UUID> = None;
        let mut cmc: Option<f32> = None;
        let mut color_indicator: Option<Vec<Color>> = None;
        let mut colors: Option<Vec<Color>> = None;
        let mut defense: Option<String> = None;
        let mut flavor_text: Option<String> = None;
        let mut illustration_id: Option<UUID> = None;
        let mut image_uris: Option<ImageURIs> = None;
        let mut layout: Option<Layout> = None;
        let mut loyalty: Option<String> = None;
        let mut mana_cost: Option<String> = None;
        let mut name: Option<String> = None;
        let mut oracle_id: Option<UUID> = None;
        let mut oracle_text: Option<String> = None;
        let mut power: Option<String> = None;
        let mut printed_name: Option<String> = None;
        let mut printed_text: Option<String> = None;
        let mut printed_type_line: Option<String> = None;
        let mut toughness: Option<String> = None;
        let mut type_line: Option<String> = None;
        let mut watermark: Option<String> = None;

        for (field, val) in fields {
            match &field[..] {
                "artist" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    artist = Some(s);
                },
                "artist_id" => {
                    artist_id = Some(UUID::deserialize(val)?);
                },
                "cmc" => {
                    if !val.is_num() {
                        return Err(ParseError::MismatchedType)
                    }

                    let n = val.unwrap_num();
                    let n = (&n).parse::<f32>()
                        .map_err(|_| ParseError::UnkownVal(n))?;
                    
                    cmc = Some(n);
                },
                "color_indicator" => {
                    let arr = Color::deserialize_array(val)?;
                    color_indicator = Some(arr);
                },
                "colors" => {
                    let arr = Color::deserialize_array(val)?;
                    colors = Some(arr);
                },
                "defense" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    defense = Some(s);
                },
                "flavor_text" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    flavor_text = Some(s);
                },
                "illustration_id" => {
                    illustration_id = Some(UUID::deserialize(val)?);
                },
                "image_uris" => {
                    image_uris = Some(ImageURIs::deserialize(val)?);
                },
                "layout" => {
                    layout = Some(Layout::deserialize(val)?);
                },
                "loyalty" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    loyalty = Some(s);
                },
                "mana_cost" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    mana_cost = Some(s);
                },
                "name" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    name = Some(s);
                },
                "oracle_id" => {
                    oracle_id = Some(UUID::deserialize(val)?);
                },
                "oracle_text" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    oracle_text = Some(s);
                },
                "power" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    power = Some(s);
                },
                "printed_name" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    printed_name = Some(s);
                },
                "printed_text" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    printed_text = Some(s);
                },
                "printed_type_line" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    printed_type_line = Some(s);
                },
                "toughness" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    toughness = Some(s);
                },
                "type_line" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    type_line = Some(s);
                },
                "watermark" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    watermark = Some(s);
                },
                "object" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    if &s[..] != "card_face" {
                        return Err(ParseError::UnkownVal(s))
                    }
                },
                _ => return Err(ParseError::UnkownVal(field)),
            }
        }

        let res = Self {
            artist,
            artist_id,
            cmc,
            color_indicator,
            colors,
            defense,
            flavor_text,
            illustration_id,
            image_uris,
            layout,
            loyalty,
            mana_cost: mana_cost.ok_or(ParseError::ValueExpected)?,
            name: name.ok_or(ParseError::ValueExpected)?,
            oracle_id,
            oracle_text,
            power,
            printed_name,
            printed_text,
            printed_type_line,
            toughness,
            type_line,
            watermark,
        };

        Ok(res)
    }
}
