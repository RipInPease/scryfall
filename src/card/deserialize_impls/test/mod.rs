use crate::deserialize::*;
use crate::card::*;
use crate::URI;

#[test]
fn color_1() {
    let w = DesValue::String(String::from("W"));
    let u = DesValue::String(String::from("U"));
    let b = DesValue::String(String::from("B"));
    let r = DesValue::String(String::from("R"));
    let g = DesValue::String(String::from("G"));
    let c = DesValue::String(String::from("C"));
    let f = DesValue::String(String::from("F"));

    let w = Color::deserialize(w);
    let u = Color::deserialize(u);
    let b = Color::deserialize(b);
    let r = Color::deserialize(r);
    let g = Color::deserialize(g);
    let c = Color::deserialize(c);
    let f = Color::deserialize(f);

    assert_eq!(w, Ok(Color::White));
    assert_eq!(u, Ok(Color::Blue));
    assert_eq!(b, Ok(Color::Black));
    assert_eq!(r, Ok(Color::Red));
    assert_eq!(g, Ok(Color::Green));
    assert_eq!(c, Ok(Color::Colorless));
    assert_eq!(f, Err(ParseError::UnkownVal(String::from("F"))));
}

#[test]
fn lang_1() {
    let en  = DesValue::String(String::from("en"));
    let es  = DesValue::String(String::from("es"));
    let fr  = DesValue::String(String::from("fr"));
    let de  = DesValue::String(String::from("de"));
    let it  = DesValue::String(String::from("it"));
    let pt  = DesValue::String(String::from("pt"));
    let ja  = DesValue::String(String::from("ja"));
    let ko  = DesValue::String(String::from("ko"));
    let ru  = DesValue::String(String::from("ru"));
    let zhs = DesValue::String(String::from("zhs"));
    let zht = DesValue::String(String::from("zht"));
    let he  = DesValue::String(String::from("he"));
    let la  = DesValue::String(String::from("la"));
    let grc = DesValue::String(String::from("grc"));
    let ar  = DesValue::String(String::from("ar"));
    let sa  = DesValue::String(String::from("sa"));
    let ph  = DesValue::String(String::from("ph"));
    let qya = DesValue::String(String::from("qya"));
    let dw  = DesValue::String(String::from("dw"));
    let fail  = DesValue::String(String::from("fail"));

    let en = Language::deserialize(en);
    let es = Language::deserialize(es);
    let fr = Language::deserialize(fr);
    let de = Language::deserialize(de);
    let it = Language::deserialize(it);
    let pt = Language::deserialize(pt);
    let ja = Language::deserialize(ja);
    let ko = Language::deserialize(ko);
    let ru = Language::deserialize(ru);
    let zhs = Language::deserialize(zhs);
    let zht = Language::deserialize(zht);
    let he = Language::deserialize(he);
    let la = Language::deserialize(la);
    let grc = Language::deserialize(grc);
    let ar = Language::deserialize(ar);
    let sa = Language::deserialize(sa);
    let ph = Language::deserialize(ph);
    let qya = Language::deserialize(qya);
    let dw = Language::deserialize(dw);
    let fail = Language::deserialize(fail);

    assert_eq!(en,  Ok(Language::EN));
    assert_eq!(es,  Ok(Language::ES));
    assert_eq!(fr,  Ok(Language::FR));
    assert_eq!(de,  Ok(Language::DE));
    assert_eq!(it,  Ok(Language::IT));
    assert_eq!(pt,  Ok(Language::PT));
    assert_eq!(ja,  Ok(Language::JA));
    assert_eq!(ko,  Ok(Language::KO));
    assert_eq!(ru,  Ok(Language::RU));
    assert_eq!(zhs, Ok(Language::ZHS));
    assert_eq!(zht, Ok(Language::ZHT));
    assert_eq!(he,  Ok(Language::HE));
    assert_eq!(la,  Ok(Language::LA));
    assert_eq!(grc, Ok(Language::GRC));
    assert_eq!(ar,  Ok(Language::AR));
    assert_eq!(sa,  Ok(Language::SA));
    assert_eq!(ph,  Ok(Language::PH));
    assert_eq!(qya, Ok(Language::QYA));
    assert_eq!(dw,  Ok(Language::DW));
    assert_eq!(fail,  Err(ParseError::UnkownVal(String::from("fail"))));
}

#[test]
fn layout_1() {
    let normal             = DesValue::String(String::from("normal"));
    let split              = DesValue::String(String::from("split"));
    let flip               = DesValue::String(String::from("flip"));
    let transform          = DesValue::String(String::from("transform"));
    let modal_dfc          = DesValue::String(String::from("modal_dfc"));
    let meld               = DesValue::String(String::from("meld"));
    let leveler            = DesValue::String(String::from("leveler"));
    let class              = DesValue::String(String::from("class"));
    let case               = DesValue::String(String::from("case"));
    let saga               = DesValue::String(String::from("saga"));
    let adventure          = DesValue::String(String::from("adventure"));
    let prepare            = DesValue::String(String::from("prepare"));
    let mutate             = DesValue::String(String::from("mutate"));
    let prototype          = DesValue::String(String::from("prototype"));
    let battle             = DesValue::String(String::from("battle"));
    let planar             = DesValue::String(String::from("planar"));
    let scheme             = DesValue::String(String::from("scheme"));
    let vanguard           = DesValue::String(String::from("vanguard"));
    let token              = DesValue::String(String::from("token"));
    let double_faced_token = DesValue::String(String::from("double_faced_token"));
    let emblem             = DesValue::String(String::from("emblem"));
    let augment            = DesValue::String(String::from("augment"));
    let host               = DesValue::String(String::from("host"));
    let art_series         = DesValue::String(String::from("art_series"));
    let reversible_card    = DesValue::String(String::from("reversible_card"));
    let fail               = DesValue::String(String::from("fail"));

    let normal             = Layout::deserialize(normal);
    let split              = Layout::deserialize(split);
    let flip               = Layout::deserialize(flip);
    let transform          = Layout::deserialize(transform);
    let modal_dfc          = Layout::deserialize(modal_dfc);
    let meld               = Layout::deserialize(meld);
    let leveler            = Layout::deserialize(leveler);
    let class              = Layout::deserialize(class);
    let case               = Layout::deserialize(case);
    let saga               = Layout::deserialize(saga);
    let adventure          = Layout::deserialize(adventure);
    let prepare            = Layout::deserialize(prepare);
    let mutate             = Layout::deserialize(mutate);
    let prototype          = Layout::deserialize(prototype);
    let battle             = Layout::deserialize(battle);
    let planar             = Layout::deserialize(planar);
    let scheme             = Layout::deserialize(scheme);
    let vanguard           = Layout::deserialize(vanguard);
    let token              = Layout::deserialize(token);
    let double_faced_token = Layout::deserialize(double_faced_token);
    let emblem             = Layout::deserialize(emblem);
    let augment            = Layout::deserialize(augment);
    let host               = Layout::deserialize(host);
    let art_series         = Layout::deserialize(art_series);
    let reversible_card    = Layout::deserialize(reversible_card);
    let fail               = Layout::deserialize(fail);

    assert_eq!(normal,             Ok(Layout::Normal));
    assert_eq!(split,              Ok(Layout::Split));
    assert_eq!(flip,               Ok(Layout::Flip));
    assert_eq!(transform,          Ok(Layout::Transform));
    assert_eq!(modal_dfc,          Ok(Layout::ModalDFC));
    assert_eq!(meld,               Ok(Layout::Meld));
    assert_eq!(leveler,            Ok(Layout::Leveler));
    assert_eq!(class,              Ok(Layout::Class));
    assert_eq!(case,               Ok(Layout::Case));
    assert_eq!(saga,               Ok(Layout::Saga));
    assert_eq!(adventure,          Ok(Layout::Adventure));
    assert_eq!(prepare,            Ok(Layout::Prepare));
    assert_eq!(mutate,             Ok(Layout::Mutate));
    assert_eq!(prototype,          Ok(Layout::Prototype));
    assert_eq!(battle,             Ok(Layout::Battle));
    assert_eq!(planar,             Ok(Layout::Planar));
    assert_eq!(scheme,             Ok(Layout::Scheme));
    assert_eq!(vanguard,           Ok(Layout::Vanguard));
    assert_eq!(token,              Ok(Layout::Token));
    assert_eq!(double_faced_token, Ok(Layout::DoubleFacedToken));
    assert_eq!(emblem,             Ok(Layout::Emblem));
    assert_eq!(augment,            Ok(Layout::Augment));
    assert_eq!(host,               Ok(Layout::Host));
    assert_eq!(art_series,         Ok(Layout::ArtSeries));
    assert_eq!(reversible_card,    Ok(Layout::ReversibleCard));
    assert_eq!(fail, Err(ParseError::UnkownVal(String::from("fail"))));
}

#[test]
fn image_uris_1() {
    let s = "
        {
            \"small\": \"\\\"https://cards.scryfall.io/small/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\\\"\",
            \"normal\": \"\\\"https://cards.scryfall.io/normal/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\\\"\",
            \"large\": \"\\\"https://cards.scryfall.io/large/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\\\"\",
            \"png\": \"\\\"https://cards.scryfall.io/png/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.png?1783941748\\\"\",
            \"art_crop\": \"\\\"https://cards.scryfall.io/art_crop/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\\\"\",
            \"border_crop\": \"\\\"https://cards.scryfall.io/border_crop/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\\\"\"
        }
    ";

    let small = Some(URI(String::from("\"https://cards.scryfall.io/small/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\"")));    
    let normal = Some(URI(String::from("\"https://cards.scryfall.io/normal/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\"")));    
    let large = Some(URI(String::from("\"https://cards.scryfall.io/large/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\"")));    
    let png = Some(URI(String::from("\"https://cards.scryfall.io/png/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.png?1783941748\"")));    
    let art_crop = Some(URI(String::from("\"https://cards.scryfall.io/art_crop/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\"")));    
    let border_crop = Some(URI(String::from("\"https://cards.scryfall.io/border_crop/front/9/e/9e17bbf7-00c0-46f2-9718-2762fd7388d3.jpg?1783941748\"")));

    let should = ImageURIs {
        small,
        normal,
        large,
        png,
        art_crop,
        border_crop,
        thumb: None,
        grid: None,
        display: None,
        crop: None,
        art: None
    };

    let tokens = parse_json_string(s.to_string()).unwrap();
    let res = ImageURIs::deserialize(tokens);
    
    assert_eq!(
        res,
        Ok(should)
    )
}

#[test]
fn card_face_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/card_face.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let card_face = CardFace::deserialize(tokens);
    assert!(card_face.is_ok());
}

#[test]
fn related_card_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/related_card.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let related_card = RelatedCard::deserialize(tokens);
    assert!(related_card.is_ok());
}

#[test]
fn legality_1() {
    let legal = DesValue::String(String::from("legal"));
    let not_legal = DesValue::String(String::from("not_legal"));
    let restricted = DesValue::String(String::from("restricted"));
    let banned = DesValue::String(String::from("banned"));
    let fail = DesValue::String(String::from("fail"));

    let legal = Legality::deserialize(legal);
    let not_legal = Legality::deserialize(not_legal);
    let restricted = Legality::deserialize(restricted);
    let banned = Legality::deserialize(banned);
    let fail = Legality::deserialize(fail);

    assert_eq!(legal, Ok(Legality::Legal));
    assert_eq!(not_legal, Ok(Legality::NotLegal));
    assert_eq!(restricted, Ok(Legality::Restricted));
    assert_eq!(banned, Ok(Legality::Banned));
    assert_eq!(fail, Err(ParseError::UnkownVal(String::from("fail"))));    
}

#[test]
fn legalities_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/legalities.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let legalities = Legalities::deserialize(tokens).unwrap();

    let should = Legalities {
        standard: Legality::NotLegal,
        future: Legality::NotLegal,
        historic: Legality::Banned,
        timeless: Legality::Legal,
        gladiator: Legality::Banned,
        pioneer: Legality::NotLegal,
        modern: Legality::Legal,
        legacy: Legality::Legal,
        pauper: Legality::NotLegal,
        vintage: Legality::Legal,
        penny: Legality::NotLegal,
        commander: Legality::Legal,
        oathbreaker: Legality::Legal,
        standardbrawl: Legality::NotLegal,
        brawl: Legality::Legal,
        competitivebrawl: Legality::Banned,
        alchemy: Legality::NotLegal,
        paupercommander: Legality::NotLegal,
        duel: Legality::Restricted,
        oldschool: Legality::NotLegal,
        premodern: Legality::NotLegal,
        predh: Legality::NotLegal,
        tlr: Legality::Restricted
    };

    assert_eq!(legalities, should);
}

#[test]
fn border_color_1() {
    let black = DesValue::String(String::from("black"));
    let white = DesValue::String(String::from("white"));
    let borderless = DesValue::String(String::from("borderless"));
    let yellow = DesValue::String(String::from("yellow"));
    let silver = DesValue::String(String::from("silver"));
    let gold = DesValue::String(String::from("gold"));
    let fail = DesValue::String(String::from("fail"));

    let black = BorderColor::deserialize(black);
    let white = BorderColor::deserialize(white);
    let borderless = BorderColor::deserialize(borderless);
    let yellow = BorderColor::deserialize(yellow);
    let silver = BorderColor::deserialize(silver);
    let gold = BorderColor::deserialize(gold);
    let fail = BorderColor::deserialize(fail);

    assert_eq!(black, Ok(BorderColor::Black));
    assert_eq!(white, Ok(BorderColor::White));
    assert_eq!(borderless, Ok(BorderColor::Borderless));
    assert_eq!(yellow, Ok(BorderColor::Yellow));
    assert_eq!(silver, Ok(BorderColor::Silver));
    assert_eq!(gold, Ok(BorderColor::Gold)); 
    assert_eq!(fail, Err(ParseError::UnkownVal(String::from("fail")))); 
}

#[test]
fn finishes_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/finishes.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let obj = tokens.unwrap_object();
    assert_eq!(obj.len(), 1);

    for token in obj {
        let should = Finishes { 
            foil: true,
            nonfoil: true,
            etched: false
        };

        let finishes = Finishes::deserialize(token.1);
        assert_eq!(finishes, Ok(should));
    }
}

#[test]
fn frame_effects_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/frame_effects.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let obj = tokens.unwrap_object();
    assert_eq!(obj.len(), 1);

    for token in obj {
        let should = FrameEffects { 
            legendary: false,
            miracle: false,
            enchantment: false,
            draft: false,
            devoid: false,
            tombstone: false,
            colorshifted: false,
            inverted: false,
            sunmoondfc: true,
            compasslanddfc: false,
            originpwdfc: false,
            mooneldrazidfc: false,
            waxingandwaningmoondfc: false,
            showcase: false,
            extendedart: false,
            companion: false,
            etched: false,
            snow: false,
            lesson: false,
            shatteredglass: false,
            convertdfc: false,
            fandfc: false,
            upsidedowndfc: false,
            spree: false,
        };

        let effects = FrameEffects::deserialize(token.1);
        assert_eq!(effects, Ok(should));
    }
}

#[test]
fn frame_1() {
    let original = DesValue::String(String::from("1993"));
    let updated_classic = DesValue::String(String::from("1997"));
    let modern = DesValue::String(String::from("2003"));
    let holo_foil_stamp = DesValue::String(String::from("2015"));
    let future = DesValue::String(String::from("future"));
    let fail = DesValue::String(String::from("fail"));

    let original = Frame::deserialize(original);
    let updated_classic = Frame::deserialize(updated_classic);
    let modern = Frame::deserialize(modern);
    let holo_foil_stamp = Frame::deserialize(holo_foil_stamp);
    let future = Frame::deserialize(future);
    let fail = Frame::deserialize(fail);

    assert_eq!(original, Ok(Frame::Original));
    assert_eq!(updated_classic, Ok(Frame::UpdatedClassic));
    assert_eq!(modern, Ok(Frame::Modern));
    assert_eq!(holo_foil_stamp, Ok(Frame::HoloFoilStamp));
    assert_eq!(future, Ok(Frame::Future));    
    assert_eq!(fail, Err(ParseError::UnkownVal(String::from("fail"))));    
}

#[test]
fn games_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/games.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let obj = tokens.unwrap_object();
    assert_eq!(obj.len(), 1);

    for token in obj {
        let should = Games { 
            paper: true,
            arena: true,
            mtgo: false,
            astral: false,
            sega: false
        };

        let finishes = Games::deserialize(token.1);
        assert_eq!(finishes, Ok(should));
    }
}

#[test]
fn image_status_1() {
    let missing = DesValue::String(String::from("missing"));
    let placeholder = DesValue::String(String::from("placeholder"));
    let lowres = DesValue::String(String::from("lowres"));
    let highres_scan = DesValue::String(String::from("highres_scan"));
    let fail = DesValue::String(String::from("fail"));

    let missing = ImageStatus::deserialize(missing);
    let placeholder = ImageStatus::deserialize(placeholder);
    let lowres = ImageStatus::deserialize(lowres);
    let highres_scan = ImageStatus::deserialize(highres_scan);
    let fail = ImageStatus::deserialize(fail);

    assert_eq!(missing, Ok(ImageStatus::Missing));
    assert_eq!(placeholder, Ok(ImageStatus::Placeholder));
    assert_eq!(lowres, Ok(ImageStatus::Lowres));
    assert_eq!(highres_scan, Ok(ImageStatus::HighRes));
    assert_eq!(fail, Err(ParseError::UnkownVal(String::from("fail"))));    
}

#[test]
fn prices_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/prices.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let prices = Prices::deserialize(tokens);

    let should = Prices {
        usd: Some(String::from("0.66")),
        usd_foil: Some(String::from("1.34")),
        usd_etched: None,
        eur: Some(String::from("0.64")),
        eur_foil: Some(String::from("1.16")),
        tix: Some(String::from("0.02")),
    };

    assert_eq!(prices, Ok(should));
}

#[test]
fn purchase_uris_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/purchase_uris.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let prices = PurchaseURIs::deserialize(tokens);

    let should = PurchaseURIs {
        tcgplayer: URI(String::from("https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&u=https%3A%2F%2Fwww.tcgplayer.com%2Fproduct%2F134858%3Fpage%3D1")),
        cardmarket: URI(String::from("https://www.cardmarket.com/en/Magic/Products?idProduct=298462&referrer=scryfall&utm_campaign=card_prices&utm_medium=text&utm_source=scryfall")),
        cardhoarder: URI(String::from("https://www.cardhoarder.com/cards/64488?affiliate_id=scryfall&ref=card-profile&utm_campaign=affiliate&utm_medium=card&utm_source=scryfall")),
    };

    assert_eq!(prices, Ok(should));
}

#[test]
fn rarity_1() {
    let common = DesValue::String(String::from("common"));
    let uncommon = DesValue::String(String::from("uncommon"));
    let rare = DesValue::String(String::from("rare"));
    let special = DesValue::String(String::from("special"));
    let mythic = DesValue::String(String::from("mythic"));
    let bonus = DesValue::String(String::from("bonus"));
    let fail = DesValue::String(String::from("fail"));

    let common = Rarity::deserialize(common);
    let uncommon = Rarity::deserialize(uncommon);
    let rare = Rarity::deserialize(rare);
    let special = Rarity::deserialize(special);
    let mythic = Rarity::deserialize(mythic);
    let bonus = Rarity::deserialize(bonus);
    let fail = Rarity::deserialize(fail);

    assert_eq!(common, Ok(Rarity::Common));
    assert_eq!(uncommon, Ok(Rarity::Uncommon));
    assert_eq!(rare, Ok(Rarity::Rare));
    assert_eq!(special, Ok(Rarity::Special));
    assert_eq!(mythic, Ok(Rarity::Mythic));
    assert_eq!(bonus, Ok(Rarity::Bonus));
    assert_eq!(fail, Err(ParseError::UnkownVal(String::from("fail"))));
}

#[test]
fn related_uris_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/related_uris.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let prices = RelatedURIs::deserialize(tokens);

    let should = RelatedURIs {
        gatherer: URI(String::from("https://gatherer.wizards.com/Pages/Card/Details.aspx?multiverseid=522262&printed=false")),
        tcgplayer_infinite_articles: URI(String::from("https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&trafcat=tcgplayer.com%2Fsearch%2Farticles&u=https%3A%2F%2Fwww.tcgplayer.com%2Fsearch%2Farticles%3FproductLineName%3Dmagic%26q%3DGalion%252C%2BElvenking%2527s%2BButler")),
        tcgplayer_infinite_decks: URI(String::from("https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&trafcat=tcgplayer.com%2Fsearch%2Fdecks&u=https%3A%2F%2Fwww.tcgplayer.com%2Fsearch%2Fdecks%3FproductLineName%3Dmagic%26q%3DGalion%252C%2BElvenking%2527s%2BButler")),
        edhrec: URI(String::from("https://edhrec.com/route/?cc=Galion%2C+Elvenking%27s+Butler")),
    };

    assert_eq!(prices, Ok(should));
}

#[test]
fn security_stamp_1() {
    let oval = DesValue::String(String::from("oval"));
    let triangle = DesValue::String(String::from("triangle"));
    let acorn = DesValue::String(String::from("acorn"));
    let circle = DesValue::String(String::from("circle"));
    let arena = DesValue::String(String::from("arena"));
    let heart = DesValue::String(String::from("heart"));
    let fail = DesValue::String(String::from("fail"));

    let oval = SecurityStamp::deserialize(oval);
    let triangle = SecurityStamp::deserialize(triangle);
    let acorn = SecurityStamp::deserialize(acorn);
    let circle = SecurityStamp::deserialize(circle);
    let arena = SecurityStamp::deserialize(arena);
    let heart = SecurityStamp::deserialize(heart);
    let fail = SecurityStamp::deserialize(fail);

    assert_eq!(oval, Ok(SecurityStamp::Oval));
    assert_eq!(triangle, Ok(SecurityStamp::Triangle));
    assert_eq!(acorn, Ok(SecurityStamp::Acorn));
    assert_eq!(circle, Ok(SecurityStamp::Circle));
    assert_eq!(arena, Ok(SecurityStamp::Arena));
    assert_eq!(heart, Ok(SecurityStamp::Heart));
    assert_eq!(fail, Err(ParseError::UnkownVal(String::from("fail"))));
}

#[test]
fn pewview_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/preview.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let preview = Preview::deserialize(tokens);

    let should = Preview {
        source: Some(String::from("Wizards of the Coast")),
        source_uri: Some(URI(String::from("https://magic.wizards.com/en/articles/archive/card-preview/challenge-accepted-2021-05-26"))),
        previewed_at: Some(String::from("2021-05-26")),
    };

    assert_eq!(preview, Ok(should));
}

#[test]
fn card_1() {
    let s = std::fs::read_to_string("src/card/deserialize_impls/test/card.json").unwrap();
    let tokens = parse_json_string(s).unwrap();

    let _ = Card::deserialize(tokens).unwrap();
}
