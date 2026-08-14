use super::CardSymbol;
use crate::scryfall_objects::*;
use crate::deserialize::{Deserialize, parse_json_string};

#[test]
fn deserialize_card_symbol_object() {
    let s = std::fs::read_to_string("src/scryfall_objects/card_symbol/test/card_symbol.json").unwrap();
    let tokens = parse_json_string(s).unwrap();
    let res = CardSymbol::deserialize(tokens);
    let res = res.unwrap();


    let should = CardSymbol {
        symbol: String::from("{T}"),
        loose_variant: None,
        english: String::from("tap this permanent"),
        transposable: false,
        represents_mana: false,
        mana_value: Some(0.0),
        cmc: Some(0.0),
        appears_in_mana_costs: false,
        funny: false,
        colors: Box::new([]),
        hybrid: false,
        phyrexian: false,
        gatherer_alternates: Some(vec![
            String::from("ocT"),
            String::from("oT"),
        ].into_boxed_slice()),
        svg_uri: Some(URI(String::from("https://svgs.scryfall.io/card-symbols/T.svg"))),
    };

    assert_eq!(res, should);
}