#[cfg(test)]
mod test;

use super::*;
use crate::deserialize::{Deserialize, DesValue, ParseError};

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