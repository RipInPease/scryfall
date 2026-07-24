use crate::deserialize::*;
use crate::card::*;

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