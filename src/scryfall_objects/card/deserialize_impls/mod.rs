#[cfg(test)]
mod test;

use super::*;
use crate::deserialize::{Deserialize, DesValue, ParseError};
use crate::scryfall_objects::*;

impl Deserialize for Color {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
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

        for (field, val) in fields {
            match &field[..] {
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
                _ => return Err(ParseError::UnkownVal(field))
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
                "object" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    if &s[..] != "card_face" {
                        return Err(ParseError::UnkownVal(s))
                    }
                },
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


impl Deserialize for RelatedCard {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_object() {
            return Err(ParseError::MismatchedType)
        }

        let fields = tokens.unwrap_object();

        let mut id:         Option<UUID>   = None;
        let mut component:  Option<String> = None;
        let mut name:       Option<String> = None;
        let mut type_line:  Option<String> = None;
        let mut uri:        Option<URI>    = None;

        for (field, val) in fields {
            match &field[..] {
                "object" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    if &s[..] != "related_card" {
                        return Err(ParseError::UnkownVal(s))
                    }
                },
                "id" => {
                    let uuid = UUID::deserialize(val)?;
                    id = Some(uuid)
                },
                "component" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    component = Some(s)
                },
                "name" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    name = Some(s)
                },
                "type_line" => {
                    if !val.is_string() {
                        return Err(ParseError::MismatchedType)
                    }

                    let s = val.unwrap_string();
                    type_line = Some(s)
                },
                "uri" => {
                    let v = URI::deserialize(val)?;
                    uri = Some(v)
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        let res = Self {
            id       : id.ok_or(ParseError::ValueExpected)?,
            component: component.ok_or(ParseError::ValueExpected)?,
            name     : name.ok_or(ParseError::ValueExpected)?,
            type_line: type_line.ok_or(ParseError::ValueExpected)?,
            uri      : uri.ok_or(ParseError::ValueExpected)?
        };

        return Ok(res)
    }
}


impl Deserialize for Legality {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_string() {
            return Err(ParseError::MismatchedType)
        }

        let s = tokens.unwrap_string();
        
        match &s[..] {
            "legal"      => Ok(Self::Legal),
            "not_legal"  => Ok(Self::NotLegal),
            "restricted" => Ok(Self::Restricted),
            "banned"     => Ok(Self::Banned),
            _            => Err(ParseError::UnkownVal(s))
        } 
    }
}


impl Deserialize for Legalities {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_object() {
            return Err(ParseError::MismatchedType)
        }

        let fields = tokens.unwrap_object();

        let mut standard: Option<Legality> = None;
        let mut future: Option<Legality> = None;
        let mut historic: Option<Legality> = None;
        let mut timeless: Option<Legality> = None;
        let mut gladiator: Option<Legality> = None;
        let mut pioneer: Option<Legality> = None;
        let mut modern: Option<Legality> = None;
        let mut legacy: Option<Legality> = None;
        let mut pauper: Option<Legality> = None;
        let mut vintage: Option<Legality> = None;
        let mut penny: Option<Legality> = None;
        let mut commander: Option<Legality> = None;
        let mut oathbreaker: Option<Legality> = None;
        let mut standardbrawl: Option<Legality> = None;
        let mut brawl: Option<Legality> = None;
        let mut competitivebrawl: Option<Legality> = None;
        let mut alchemy: Option<Legality> = None;
        let mut paupercommander: Option<Legality> = None;
        let mut duel: Option<Legality> = None;
        let mut oldschool: Option<Legality> = None;
        let mut premodern: Option<Legality> = None;
        let mut predh: Option<Legality> = None;
        let mut tlr: Option<Legality> = None;

        for (field, val) in fields {
            match &field[..] {
                "standard" => {
                    let legality = Legality::deserialize(val)?;
                    standard = Some(legality);
                },
                "future" => {
                    let legality = Legality::deserialize(val)?;
                    future = Some(legality);
                },
                "historic" => {
                    let legality = Legality::deserialize(val)?;
                    historic = Some(legality);
                },
                "timeless" => {
                    let legality = Legality::deserialize(val)?;
                    timeless = Some(legality);
                },
                "gladiator" => {
                    let legality = Legality::deserialize(val)?;
                    gladiator = Some(legality);
                },
                "pioneer" => {
                    let legality = Legality::deserialize(val)?;
                    pioneer = Some(legality);
                },
                "modern" => {
                    let legality = Legality::deserialize(val)?;
                    modern = Some(legality);
                },
                "legacy" => {
                    let legality = Legality::deserialize(val)?;
                    legacy = Some(legality);
                },
                "pauper" => {
                    let legality = Legality::deserialize(val)?;
                    pauper = Some(legality);
                },
                "vintage" => {
                    let legality = Legality::deserialize(val)?;
                    vintage = Some(legality);
                },
                "penny" => {
                    let legality = Legality::deserialize(val)?;
                    penny = Some(legality);
                },
                "commander" => {
                    let legality = Legality::deserialize(val)?;
                    commander = Some(legality);
                },
                "oathbreaker" => {
                    let legality = Legality::deserialize(val)?;
                    oathbreaker = Some(legality);
                },
                "standardbrawl" => {
                    let legality = Legality::deserialize(val)?;
                    standardbrawl = Some(legality);
                },
                "brawl" => {
                    let legality = Legality::deserialize(val)?;
                    brawl = Some(legality);
                },
                "competitivebrawl" => {
                    let legality = Legality::deserialize(val)?;
                    competitivebrawl = Some(legality);
                },
                "alchemy" => {
                    let legality = Legality::deserialize(val)?;
                    alchemy = Some(legality);
                },
                "paupercommander" => {
                    let legality = Legality::deserialize(val)?;
                    paupercommander = Some(legality);
                },
                "duel" => {
                    let legality = Legality::deserialize(val)?;
                    duel = Some(legality);
                },
                "oldschool" => {
                    let legality = Legality::deserialize(val)?;
                    oldschool = Some(legality);
                },
                "premodern" => {
                    let legality = Legality::deserialize(val)?;
                    premodern = Some(legality);
                },
                "predh" => {
                    let legality = Legality::deserialize(val)?;
                    predh = Some(legality);
                },
                "tlr" => {
                    let legality = Legality::deserialize(val)?;
                    tlr = Some(legality);
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        let res = Self {
            standard: standard.ok_or(ParseError::ValueExpected)?,
            future: future.ok_or(ParseError::ValueExpected)?,
            historic: historic.ok_or(ParseError::ValueExpected)?,
            timeless: timeless.ok_or(ParseError::ValueExpected)?,
            gladiator: gladiator.ok_or(ParseError::ValueExpected)?,
            pioneer: pioneer.ok_or(ParseError::ValueExpected)?,
            modern: modern.ok_or(ParseError::ValueExpected)?,
            legacy: legacy.ok_or(ParseError::ValueExpected)?,
            pauper: pauper.ok_or(ParseError::ValueExpected)?,
            vintage: vintage.ok_or(ParseError::ValueExpected)?,
            penny: penny.ok_or(ParseError::ValueExpected)?,
            commander: commander.ok_or(ParseError::ValueExpected)?,
            oathbreaker: oathbreaker.ok_or(ParseError::ValueExpected)?,
            standardbrawl: standardbrawl.ok_or(ParseError::ValueExpected)?,
            brawl: brawl.ok_or(ParseError::ValueExpected)?,
            competitivebrawl: competitivebrawl.ok_or(ParseError::ValueExpected)?,
            alchemy: alchemy.ok_or(ParseError::ValueExpected)?,
            paupercommander: paupercommander.ok_or(ParseError::ValueExpected)?,
            duel: duel.ok_or(ParseError::ValueExpected)?,
            oldschool: oldschool.ok_or(ParseError::ValueExpected)?,
            premodern: premodern.ok_or(ParseError::ValueExpected)?,
            predh: predh.ok_or(ParseError::ValueExpected)?,
            tlr: tlr.ok_or(ParseError::ValueExpected)?,
        };

        return Ok(res)
    }
}


impl Deserialize for BorderColor {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_string() {
            return Err(ParseError::MismatchedType)
        }

        let s = tokens.unwrap_string();

        match &s[..] {
            "black"      => Ok(Self::Black),
            "white"      => Ok(Self::White),
            "borderless" => Ok(Self::Borderless),
            "yellow"     => Ok(Self::Yellow),
            "silver"     => Ok(Self::Silver),
            "gold"       => Ok(Self::Gold),
            _            => Err(ParseError::UnkownVal(s))
        }
    }
}


impl Deserialize for Finishes {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_array() {
            return Err(ParseError::MismatchedType)
        }

        let arr = tokens.unwrap_array();

        let mut foil    = false;
        let mut nonfoil = false;
        let mut etched  = false;

        for v in arr {
            if !v.is_string() {
                return Err(ParseError::MismatchedType)
            }

            let s = v.unwrap_string();
            match &s[..] {
                "foil"    => foil    = true,
                "nonfoil" => nonfoil = true,
                "etched"  => etched  = true,
                _         => return Err(ParseError::UnkownVal(s))
            }
        }

        Ok(Self { foil, nonfoil, etched })
    }
}


impl Deserialize for FrameEffects {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_array() {
            return Err(ParseError::MismatchedType)
        }

        let arr = tokens.unwrap_array();

        let mut legendary               = false;
        let mut miracle                 = false;
        let mut enchantment             = false;
        let mut draft                   = false;
        let mut devoid                  = false;
        let mut tombstone               = false;
        let mut colorshifted            = false;
        let mut inverted                = false;
        let mut sunmoondfc              = false;
        let mut compasslanddfc          = false;
        let mut originpwdfc             = false;
        let mut mooneldrazidfc          = false;
        let mut waxingandwaningmoondfc  = false;
        let mut showcase                = false;
        let mut extendedart             = false;
        let mut companion               = false;
        let mut etched                  = false;
        let mut snow                    = false;
        let mut lesson                  = false;
        let mut shatteredglass          = false;
        let mut convertdfc              = false;
        let mut fandfc                  = false;
        let mut upsidedowndfc           = false;
        let mut spree                   = false;

        for v in arr {
            if !v.is_string() {
                return Err(ParseError::MismatchedType)
            }

            let s = v.unwrap_string();
            match &s[..] {
                "legendary"                 => legendary = true,
                "miracle"                   => miracle = true,
                "enchantment"               => enchantment = true,
                "draft"                     => draft = true,
                "devoid"                    => devoid = true,
                "tombstone"                 => tombstone = true,
                "colorshifted"              => colorshifted = true,
                "inverted"                  => inverted = true,
                "sunmoondfc"                => sunmoondfc = true,
                "compasslanddfc"            => compasslanddfc = true,
                "originpwdfc"               => originpwdfc = true,
                "mooneldrazidfc"            => mooneldrazidfc = true,
                "waxingandwaningmoondfc"    => waxingandwaningmoondfc = true,
                "showcase"                  => showcase = true,
                "extendedart"               => extendedart = true,
                "companion"                 => companion = true,
                "etched"                    => etched = true,
                "snow"                      => snow = true,
                "lesson"                    => lesson = true,
                "shatteredglass"            => shatteredglass = true,
                "convertdfc"                => convertdfc = true,
                "fandfc"                    => fandfc = true,
                "upsidedowndfc"             => upsidedowndfc = true,
                "spree"                     => spree = true,
                _                           => return Err(ParseError::UnkownVal(s))
            }
        }

        Ok(Self {
            legendary,
            miracle,
            enchantment,
            draft,
            devoid,
            tombstone,
            colorshifted,
            inverted,
            sunmoondfc,
            compasslanddfc,
            originpwdfc,
            mooneldrazidfc,
            waxingandwaningmoondfc,
            showcase,
            extendedart,
            companion,
            etched,
            snow,
            lesson,
            shatteredglass,
            convertdfc,
            fandfc,
            upsidedowndfc,
            spree,
        })
    }
}


impl Deserialize for Frame {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_string() {
            return Err(ParseError::MismatchedType)
        }

        let s = tokens.unwrap_string();

        match &s[..] {
            "1993"   => Ok(Self::Original),
            "1997"   => Ok(Self::UpdatedClassic),
            "2003"   => Ok(Self::Modern),
            "2015"   => Ok(Self::HoloFoilStamp),
            "future" => Ok(Self::Future),
            _        => Err(ParseError::UnkownVal(s))
        }
    }
}


impl Deserialize for Games {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_array() {
            return Err(ParseError::MismatchedType)
        }

        let arr = tokens.unwrap_array();

        let mut paper  = false;
        let mut arena  = false;
        let mut mtgo   = false;
        let mut astral = false;
        let mut sega   = false;

        for v in arr {
            if !v.is_string() {
                return Err(ParseError::MismatchedType)
            }

            let s = v.unwrap_string();
            match &s[..] {
                "paper"  => paper = true,
                "arena"  => arena = true,
                "mtgo"   => mtgo = true,
                "astral" => astral = true,
                "sega"   => sega = true,
                _        => return Err(ParseError::UnkownVal(s))
            }
        }

        Ok(Self { 
            paper,
            arena,
            mtgo,
            astral,
            sega
        })
    }
}


impl Deserialize for ImageStatus {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_string() {
            return Err(ParseError::MismatchedType)
        }

        let s = tokens.unwrap_string();

        match &s[..] {
            "missing"      => Ok(Self::Missing),
            "placeholder"  => Ok(Self::Placeholder),
            "lowres"       => Ok(Self::Lowres),
            "highres_scan" => Ok(Self::HighRes),
            _              => Err(ParseError::UnkownVal(s))
        }
    }
}


impl Deserialize for Prices {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_object() {
            return Err(ParseError::MismatchedType)
        }

        let fields = tokens.unwrap_object();

        let mut usd:        Option<Option<String>> = None;
        let mut usd_foil:   Option<Option<String>> = None;
        let mut usd_etched: Option<Option<String>> = None;
        let mut eur:        Option<Option<String>> = None;
        let mut eur_foil:   Option<Option<String>> = None;
        let mut tix:        Option<Option<String>> = None;

        for (field, val) in fields {
            match &field[..] {
                "usd" => {
                    if usd.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else if val.is_null() {
                        usd = Some(None)
                    } else if val.is_string() {
                        let s = val.unwrap_string();
                        usd = Some(Some(s));
                    } else {
                        return Err(ParseError::MismatchedType)
                    }
                }
                "usd_foil" => {
                    if usd_foil.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else if val.is_null() {
                        usd_foil = Some(None)
                    } else if val.is_string() {
                        let s = val.unwrap_string();
                        usd_foil = Some(Some(s));
                    } else {
                        return Err(ParseError::MismatchedType)
                    }
                }
                "usd_etched" => {
                    if usd_etched.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else if val.is_null() {
                        usd_etched = Some(None)
                    } else if val.is_string() {
                        let s = val.unwrap_string();
                        usd_etched = Some(Some(s));
                    } else {
                        return Err(ParseError::MismatchedType)
                    }
                }
                "eur" => {
                    if eur.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else if val.is_null() {
                        eur = Some(None)
                    } else if val.is_string() {
                        let s = val.unwrap_string();
                        eur = Some(Some(s));
                    } else {
                        return Err(ParseError::MismatchedType)
                    }
                }
                "eur_foil" => {
                    if eur_foil.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else if val.is_null() {
                        eur_foil = Some(None)
                    } else if val.is_string() {
                        let s = val.unwrap_string();
                        eur_foil = Some(Some(s));
                    } else {
                        return Err(ParseError::MismatchedType)
                    }
                }
                "tix" => {
                    if tix.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else if val.is_null() {
                        tix = Some(None)
                    } else if val.is_string() {
                        let s = val.unwrap_string();
                        tix = Some(Some(s));
                    } else {
                        return Err(ParseError::MismatchedType)
                    }
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Ok(Self {
            usd:        usd.ok_or(ParseError::ValueExpected)?,
            usd_foil:   usd_foil.ok_or(ParseError::ValueExpected)?,
            usd_etched: usd_etched.ok_or(ParseError::ValueExpected)?,
            eur:        eur.ok_or(ParseError::ValueExpected)?,
            eur_foil:   eur_foil.ok_or(ParseError::ValueExpected)?,
            tix:        tix.ok_or(ParseError::ValueExpected)?,
        })
    }
}


impl Deserialize for PurchaseURIs {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_object() {
            return Err(ParseError::MismatchedType)
        }

        let fields = tokens.unwrap_object();

        let mut tcgplayer: Option<URI> = None; 
        let mut cardmarket: Option<URI> = None; 
        let mut cardhoarder: Option<URI> = None;

        for (field, val) in fields {
            match &field[..] {
                "tcgplayer" => {
                    if tcgplayer.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else {
                        let uri = URI::deserialize(val)?;
                        tcgplayer = Some(uri)
                    }
                }
                "cardmarket" => {
                    if cardmarket.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else {
                        let uri = URI::deserialize(val)?;
                        cardmarket = Some(uri)
                    }
                }
                "cardhoarder" => {
                    if cardhoarder.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else {
                        let uri = URI::deserialize(val)?;
                        cardhoarder = Some(uri)
                    }
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Ok(Self {
            tcgplayer:   tcgplayer.ok_or(ParseError::ValueExpected)?,  
            cardmarket:  cardmarket.ok_or(ParseError::ValueExpected)?,  
            cardhoarder: cardhoarder.ok_or(ParseError::ValueExpected)?,
        })
    }
}


impl Deserialize for Rarity {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_string() {
            return Err(ParseError::MismatchedType)
        }

        let s = tokens.unwrap_string();

        match &s[..] {
            "common"    => Ok(Self::Common),
            "uncommon"  => Ok(Self::Uncommon),
            "rare"      => Ok(Self::Rare),
            "special"   => Ok(Self::Special),
            "mythic"    => Ok(Self::Mythic),
            "bonus"     => Ok(Self::Bonus),
            _           => Err(ParseError::UnkownVal(s))
        }
    }
}


impl Deserialize for RelatedURIs {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_object() {
            return Err(ParseError::MismatchedType)
        }

        let fields = tokens.unwrap_object();

        let mut gatherer: Option<URI> = None; 
        let mut tcgplayer_infinite_articles: Option<URI> = None; 
        let mut tcgplayer_infinite_decks: Option<URI> = None; 
        let mut edhrec: Option<URI> = None;

        for (field, val) in fields {
            match &field[..] {
                "gatherer" => {
                    if gatherer.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else {
                        let uri = URI::deserialize(val)?;
                        gatherer = Some(uri)
                    }
                },
                "tcgplayer_infinite_articles" => {
                    if tcgplayer_infinite_articles.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else {
                        let uri = URI::deserialize(val)?;
                        tcgplayer_infinite_articles = Some(uri)
                    }
                }
                "tcgplayer_infinite_decks" => {
                    if tcgplayer_infinite_decks.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else {
                        let uri = URI::deserialize(val)?;
                        tcgplayer_infinite_decks = Some(uri)
                    }
                }
                "edhrec" => {
                    if edhrec.is_some() {
                        return Err(ParseError::DuplicateValue)
                    } else {
                        let uri = URI::deserialize(val)?;
                        edhrec = Some(uri)
                    }
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Ok(Self {
            gatherer:                    gatherer.ok_or(ParseError::ValueExpected)?,
            tcgplayer_infinite_articles: tcgplayer_infinite_articles.ok_or(ParseError::ValueExpected)?,  
            tcgplayer_infinite_decks:    tcgplayer_infinite_decks.ok_or(ParseError::ValueExpected)?,  
            edhrec:                      edhrec.ok_or(ParseError::ValueExpected)?,
        })
    }
}


impl Deserialize for SecurityStamp {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        if !tokens.is_string() {
            return Err(ParseError::MismatchedType)
        }

        let s = tokens.unwrap_string();

        match &s[..] {
            "oval"    => Ok(Self::Oval),
            "triangle"  => Ok(Self::Triangle),
            "acorn"      => Ok(Self::Acorn),
            "circle"   => Ok(Self::Circle),
            "arena"    => Ok(Self::Arena),
            "heart"     => Ok(Self::Heart),
            _           => Err(ParseError::UnkownVal(s))
        }
    }
}


impl Deserialize for Preview {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        let fields = tokens.object_or(ParseError::MismatchedType)?;

        let mut previewed_at: Option<String> = None;
        let mut source_uri: Option<URI> = None;
        let mut source: Option<String> = None;

        for (field, val) in fields {
            match  &field[..] {
                "previewed_at" => {
                    if previewed_at.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    previewed_at = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "source_uri" => {
                    if source_uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let u = URI::deserialize(val)?;
                    source_uri = Some(u)
                },
                "source" => {
                    if source.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    source = Some(val.string_or(ParseError::MismatchedType)?)
                },
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        Ok(Self {
            source_uri,
            previewed_at,
            source,
        })
    }
}


impl Deserialize for Card {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        let fields = tokens.object_or(ParseError::MismatchedType)?;

        let mut arena_id: Option<i32> = None;
        let mut id: Option<UUID> = None;
        let mut lang: Option<Language> = None;
        let mut mtgo_id: Option<i32> = None;
        let mut mtgo_foil_id: Option<i32> = None;
        let mut multiverse_ids: Option<Vec<i32>> = None;
        let mut resource_id: Option<String> = None;
        let mut tcgplayer_id: Option<i32> = None;
        let mut tcgplayer_etched_id: Option<i32> = None;
        let mut cardmarket_id: Option<i32> = None;
        let mut layout: Option<Layout> = None;
        let mut oracle_id: Option<UUID> = None;
        let mut prints_search_uri: Option<URI> = None;
        let mut rulings_uri: Option<URI> = None;
        let mut scryfall_uri: Option<URI> = None;
        let mut uri: Option<URI> = None;

        let mut all_parts: Option<Vec<RelatedCard>> = None;
        let mut card_faces: Option<Vec<CardFace>> = None;
        let mut cmc: Option<f32> = None;
        let mut color_identity: Option<Vec<Color>> = None;
        let mut color_indicator: Option<Vec<Color>> = None;
        let mut colors: Option<Vec<Color>> = None;
        let mut defense: Option<String> = None;
        let mut edhrec_rank: Option<i32> = None;
        let mut game_changer: Option<bool> = None;
        let mut hand_modifier: Option<String> = None;
        let mut keywords: Option<Vec<String>> = None;
        let mut legalities: Option<Legalities> = None;
        let mut life_modifier: Option<String> = None;
        let mut loyalty: Option<String> = None;
        let mut mana_cost: Option<String> = None;
        let mut name: Option<String> = None;
        let mut oracle_text: Option<String> = None;
        let mut penny_rank: Option<i32> = None;
        let mut power: Option<String> = None;
        let mut produced_mana: Option<Vec<Color>> = None;
        let mut reserved: Option<bool> = None;
        let mut toughness: Option<String> = None;
        let mut type_line: Option<String> = None;

        let mut artist: Option<String> = None;
        let mut artist_ids: Option<Vec<UUID>> = None;
        let mut attraction_lights: Option<Vec<i32>> = None;
        let mut booster: Option<bool> = None;
        let mut border_color: Option<BorderColor> = None;
        let mut card_back_id: Option<UUID> = None;
        let mut collector_number: Option<String> = None;
        let mut content_warning: Option<bool> = None;
        let mut digital: Option<bool> = None;
        let mut finishes: Option<Finishes> = None;
        let mut flavor_name: Option<String> = None;
        let mut flavor_text: Option<String> = None;
        let mut frame_effects: Option<FrameEffects> = None;
        let mut frame: Option<Frame> = None;
        let mut full_art: Option<bool> = None;
        let mut games: Option<Games> = None;
        let mut highres_image: Option<bool> = None;
        let mut illustration_id: Option<UUID> = None;
        let mut image_status: Option<ImageStatus> = None;
        let mut image_updated_at: Option<String> = None;
        let mut image_uris: Option<ImageURIs> = None;
        let mut oversized: Option<bool> = None;
        let mut prices: Option<Prices> = None;
        let mut printed_name: Option<String> = None;
        let mut printed_text: Option<String> = None;
        let mut printed_type_line: Option<String> = None;
        let mut promo: Option<bool> = None;
        let mut promo_types: Option<Vec<String>> = None;
        let mut purchase_uris: Option<PurchaseURIs> = None;
        let mut rarity: Option<Rarity> = None;
        let mut related_uris: Option<RelatedURIs> = None;
        let mut released_at: Option<String> = None;
        let mut reprint: Option<bool> = None;
        let mut scryfall_set_uri: Option<URI> = None;
        let mut set_name: Option<String> = None;
        let mut set_search_uri: Option<URI> = None;
        let mut set_type: Option<String> = None;
        let mut set_uri: Option<URI> = None;
        let mut set: Option<String> = None;
        let mut set_id: Option<UUID> = None;
        let mut story_spotlight: Option<bool> = None;
        let mut textless: Option<bool> = None;
        let mut variation: Option<bool> = None;
        let mut variation_of: Option<UUID> = None;
        let mut security_stamp: Option<SecurityStamp> = None;
        let mut watermark: Option<String> = None;
        let mut preview: Option<Preview> = None;


        for (field, val) in fields {
            match &field[..] {
                // Foil and Nonfoil are under the object Finishes
                "foil" | "nonfoil" => {}
                "object" => {
                    let s = val.string_or(ParseError::MismatchedType)?;
                    if &s != "card" {
                        return Err(ParseError::UnkownVal(s));
                    }
                },
                "arena_id" => {
                    if arena_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    arena_id = Some(n)
                },
                "id" => {
                    if id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let uuid = UUID::deserialize(val)?;
                    id = Some(uuid);
                },
                "lang" => {
                    if lang.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let language = Language::deserialize(val)?;
                    lang = Some(language);
                },
                "mtgo_id" => {
                    if mtgo_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    mtgo_id = Some(n)
                },
                "mtgo_foil_id" => {
                    if mtgo_foil_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    mtgo_foil_id = Some(n)
                },
                "multiverse_ids" => {
                    if multiverse_ids.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        let n = v.num_or(ParseError::MismatchedType)?;
                        let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                        values.push(n);
                    }

                    multiverse_ids = Some(values)
                },
                "resource_id" => {
                    if resource_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    resource_id = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "tcgplayer_id" => {
                    if tcgplayer_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    tcgplayer_id = Some(n)
                },
                "tcgplayer_etched_id" => {
                    if tcgplayer_etched_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    tcgplayer_etched_id = Some(n)
                },
                "cardmarket_id" => {
                    if cardmarket_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    cardmarket_id = Some(n)
                },
                "layout" => {
                    if layout.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let l = Layout::deserialize(val)?;
                    layout = Some(l)
                },
                "oracle_id" => {
                    if oracle_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let uuid = UUID::deserialize(val)?;
                    oracle_id = Some(uuid)
                },
                "prints_search_uri" => {
                    if prints_search_uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let uri = URI::deserialize(val)?;
                    prints_search_uri = Some(uri)
                },
                "rulings_uri" => {
                    if rulings_uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let uri = URI::deserialize(val)?;
                    rulings_uri = Some(uri)
                },
                "scryfall_uri" => {
                    if scryfall_uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let uri = URI::deserialize(val)?;
                    scryfall_uri = Some(uri)
                },
                "uri" => {
                    if uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let u = URI::deserialize(val)?;
                    uri = Some(u)
                },
                "all_parts" => {
                    if all_parts.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(RelatedCard::deserialize(v)?);
                    }

                    all_parts = Some(values)
                },
                "card_faces" => {
                    if card_faces.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(CardFace::deserialize(v)?);
                    }

                    card_faces = Some(values)
                },
                "cmc" => {
                    if cmc.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<f32>().or(Err(ParseError::UnkownVal(n)))?;
                    cmc = Some(n)
                },
                "color_identity" => {
                    if color_identity.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(Color::deserialize(v)?);
                    }

                    color_identity = Some(values)
                },
                "color_indicator" => {
                    if color_indicator.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(Color::deserialize(v)?);
                    }

                    color_indicator = Some(values)
                },
                "colors" => {
                    if colors.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(Color::deserialize(v)?);
                    }

                    colors = Some(values)
                },
                "defense" => {
                    if defense.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    defense = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "edhrec_rank" => {
                    if edhrec_rank.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    edhrec_rank = Some(n)
                },
                "game_changer" => {
                    if game_changer.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    game_changer = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "hand_modifier" => {
                    if hand_modifier.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    hand_modifier = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "keywords" => {
                    if keywords.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(v.string_or(ParseError::MismatchedType)?);
                    }

                    keywords = Some(values)
                },
                "legalities" => {
                    if legalities.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let l = Legalities::deserialize(val)?;
                    legalities = Some(l)
                },
                "life_modifier" => {
                    if life_modifier.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    life_modifier = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "loyalty" => {
                    if loyalty.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    loyalty = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "mana_cost" => {
                    if mana_cost.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    mana_cost = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "name" => {
                    if name.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    name = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "oracle_text" => {
                    if oracle_text.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    oracle_text = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "penny_rank" => {
                    if penny_rank.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let n = val.num_or(ParseError::MismatchedType)?;
                    let n = n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?;
                    penny_rank = Some(n)
                },
                "power" => {
                    if power.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    power = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "produced_mana" => {
                    if produced_mana.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(Color::deserialize(v)?);
                    }

                    produced_mana = Some(values)
                },
                "reserved" => {
                    if reserved.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    reserved = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "toughness" => {
                    if toughness.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    toughness = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "type_line" => {
                    if type_line.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    type_line = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "artist" => {
                    if artist.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    artist = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "artist_ids" => {
                    if artist_ids.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(UUID::deserialize(v)?);
                    }

                    artist_ids = Some(values)
                },
                "attraction_lights" => {
                    if attraction_lights.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        let n = v.num_or(ParseError::MismatchedType)?;
                        values.push(n.parse::<i32>().or(Err(ParseError::UnkownVal(n)))?);
                    }

                    attraction_lights = Some(values)
                },
                "booster" => {
                    if booster.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    booster = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "border_color" => {
                    if border_color.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let b = BorderColor::deserialize(val)?;
                    border_color = Some(b)
                },
                "card_back_id" => {
                    if card_back_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let id = UUID::deserialize(val)?;
                    card_back_id = Some(id)
                },
                "collector_number" => {
                    if collector_number.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    collector_number = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "content_warning" => {
                    if content_warning.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    content_warning = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "digital" => {
                    if digital.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    digital = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "finishes" => {
                    if finishes.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let f = Finishes::deserialize(val)?;
                    finishes = Some(f)
                },
                "flavor_name" => {
                    if flavor_name.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    flavor_name = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "flavor_text" => {
                    if flavor_text.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    flavor_text = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "frame_effects" => {
                    if frame_effects.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let f = FrameEffects::deserialize(val)?;
                    frame_effects = Some(f)
                },
                "frame" => {
                    if frame.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let f = Frame::deserialize(val)?;
                    frame = Some(f)
                },
                "full_art" => {
                    if full_art.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    full_art = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "games" => {
                    if games.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let g = Games::deserialize(val)?;
                    games = Some(g)
                },
                "highres_image" => {
                    if highres_image.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    highres_image = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "illustration_id" => {
                    if illustration_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let id = UUID::deserialize(val)?;
                    illustration_id = Some(id)
                },
                "image_status" => {
                    if image_status.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let i = ImageStatus::deserialize(val)?;
                    image_status = Some(i)
                },
                "image_updated_at" => {
                    if image_updated_at.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let s = val.string_or(ParseError::MismatchedType)?;
                    image_updated_at = Some(s)
                },
                "image_uris" => {
                    if image_uris.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let i = ImageURIs::deserialize(val)?;
                    image_uris = Some(i)
                },
                "oversized" => {
                    if oversized.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    oversized = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "prices" => {
                    if prices.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let p = Prices::deserialize(val)?;
                    prices = Some(p)
                },
                "printed_name" => {
                    if printed_name.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    printed_name = Some(val.string_or(ParseError::MismatchedType)?)
                }
                "printed_text" => {
                    if printed_text.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    printed_text = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "printed_type_line" => {
                    if printed_type_line.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    printed_type_line = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "promo" => {
                    if promo.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    promo = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "promo_types" => {
                    if promo_types.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }

                    let arr = val.array_or(ParseError::MismatchedType)?;
                    let mut values = Vec::new();

                    for v in arr.into_iter() {
                        values.push(v.string_or(ParseError::MismatchedType)?);
                    }

                    promo_types = Some(values)
                },
                "purchase_uris" => {
                    if purchase_uris.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let p = PurchaseURIs::deserialize(val)?;
                    purchase_uris = Some(p)
                },
                "rarity" => {
                    if rarity.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let r = Rarity::deserialize(val)?;
                    rarity = Some(r)
                },
                "related_uris" => {
                    if related_uris.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let r = RelatedURIs::deserialize(val)?;
                    related_uris = Some(r)
                },
                "released_at" => {
                    if released_at.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    released_at = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "reprint" => {
                    if reprint.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    reprint = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "scryfall_set_uri" => {
                    if scryfall_set_uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let u = URI::deserialize(val)?;
                    scryfall_set_uri = Some(u)
                },
                "set_name" => {
                    if set_name.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    set_name = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "set_search_uri" => {
                    if set_search_uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let u = URI::deserialize(val)?;
                    set_search_uri = Some(u)
                },
                "set_type" => {
                    if set_type.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    set_type = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "set_uri" => {
                    if set_uri.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let u = URI::deserialize(val)?;
                    set_uri = Some(u)
                },
                "set" => {
                    if set.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    set = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "set_id" => {
                    if set_id.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let id = UUID::deserialize(val)?;
                    set_id = Some(id)
                },
                "story_spotlight" => {
                    if story_spotlight.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    story_spotlight = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "textless" => {
                    if textless.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    textless = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "variation" => {
                    if variation.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    variation = Some(val.bool_or(ParseError::MismatchedType)?)
                },
                "variation_of" => {
                    if variation_of.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let id = UUID::deserialize(val)?;
                    variation_of = Some(id)
                },
                "security_stamp" => {
                    if security_stamp.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let s = SecurityStamp::deserialize(val)?;
                    security_stamp = Some(s)
                },
                "watermark" => {
                    if watermark.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    watermark = Some(val.string_or(ParseError::MismatchedType)?)
                },
                "preview" => {
                    if preview.is_some() {
                        return Err(ParseError::DuplicateValue)
                    }
                    let p = Preview::deserialize(val)?;
                    preview = Some(p);
                }
                _ => return Err(ParseError::UnkownVal(field))
            }
        }

        
        let core_fields = CardCore {
            arena_id,
            id: id.ok_or(ParseError::ValueExpected)?,
            lang: lang.ok_or(ParseError::ValueExpected)?,
            mtgo_id,
            mtgo_foil_id,
            multiverse_ids,
            resource_id,
            tcgplayer_id,
            tcgplayer_etched_id,
            cardmarket_id,
            layout: layout.ok_or(ParseError::ValueExpected)?,
            oracle_id,
            prints_search_uri: prints_search_uri.ok_or(ParseError::ValueExpected)?,
            rulings_uri: rulings_uri.ok_or(ParseError::ValueExpected)?,
            scryfall_uri: scryfall_uri.ok_or(ParseError::ValueExpected)?,
            uri: uri.ok_or(ParseError::ValueExpected)?
        };

        let gameplay = CardGameplay {
            all_parts,
            card_faces,
            cmc: cmc.ok_or(ParseError::ValueExpected)?,
            color_identity: color_identity.ok_or(ParseError::ValueExpected)?,
            color_indicator,
            colors,
            defense,
            edhrec_rank,
            game_changer,
            hand_modifier,
            keywords: keywords.ok_or(ParseError::ValueExpected)?,
            legalities: legalities.ok_or(ParseError::ValueExpected)?,
            life_modifier,
            loyalty,
            mana_cost,
            name: name.ok_or(ParseError::ValueExpected)?,
            oracle_text,
            penny_rank,
            power,
            produced_mana,
            reserved: reserved.ok_or(ParseError::ValueExpected)?,
            toughness,
            type_line: type_line.ok_or(ParseError::ValueExpected)?
        };

        let print_fields = CardPrint {
            artist,
            artist_ids,
            attraction_lights,
            booster: booster.ok_or(ParseError::ValueExpected)?,
            border_color: border_color.ok_or(ParseError::ValueExpected)?,
            card_back_id: card_back_id.ok_or(ParseError::ValueExpected)?,
            collector_number: collector_number.ok_or(ParseError::ValueExpected)?,
            content_warning,
            digital: digital.ok_or(ParseError::ValueExpected)?,
            finishes: finishes.ok_or(ParseError::ValueExpected)?,
            flavor_name,
            flavor_text,
            frame_effects: frame_effects.ok_or(ParseError::ValueExpected)?,
            frame: frame.ok_or(ParseError::ValueExpected)?,
            full_art: full_art.ok_or(ParseError::ValueExpected)?,
            games: games.ok_or(ParseError::ValueExpected)?,
            highres_image: highres_image.ok_or(ParseError::ValueExpected)?,
            illustration_id,
            image_status: image_status.ok_or(ParseError::ValueExpected)?,
            image_updated_at: image_updated_at.ok_or(ParseError::ValueExpected)?,
            image_uris,
            oversized: oversized.ok_or(ParseError::ValueExpected)?,
            prices: prices.ok_or(ParseError::ValueExpected)?,
            printed_name,
            printed_text,
            printed_type_line,
            promo: promo.ok_or(ParseError::ValueExpected)?,
            promo_types,
            purchase_uris: purchase_uris.ok_or(ParseError::ValueExpected)?,
            rarity: rarity.ok_or(ParseError::ValueExpected)?,
            related_uris: related_uris.ok_or(ParseError::ValueExpected)?,
            released_at: released_at.ok_or(ParseError::ValueExpected)?,
            reprint: reprint.ok_or(ParseError::ValueExpected)?,
            scryfall_set_uri: scryfall_set_uri.ok_or(ParseError::ValueExpected)?,
            set_name: set_name.ok_or(ParseError::ValueExpected)?,
            set_search_uri: set_search_uri.ok_or(ParseError::ValueExpected)?,
            set_type: set_type.ok_or(ParseError::ValueExpected)?,
            set_uri: set_uri.ok_or(ParseError::ValueExpected)?,
            set: set.ok_or(ParseError::ValueExpected)?,
            set_id: set_id.ok_or(ParseError::ValueExpected)?,
            story_spotlight: story_spotlight.ok_or(ParseError::ValueExpected)?,
            textless: textless.ok_or(ParseError::ValueExpected)?,
            variation: variation.ok_or(ParseError::ValueExpected)?,
            variation_of: variation_of,
            security_stamp,
            watermark,
            preview: preview.ok_or(ParseError::ValueExpected)?,
        };
    
        Ok(Self { core_fields, gameplay, print_fields })
    }
}
