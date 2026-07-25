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
