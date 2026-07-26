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
                "astral" => astral = true,
                "sega"   => sega = true,
                _        => return Err(ParseError::UnkownVal(s))
            }
        }

        Ok(Self { 
            paper,
            arena,
            astral,
            sega
        })
    }
}
