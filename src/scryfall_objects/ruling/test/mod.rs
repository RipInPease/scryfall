use crate::deserialize::{Deserialize, parse_json_string};
use super::Ruling;

#[test]
fn deserialize_ruling_object_1() {
    let s = std::fs::read_to_string("src/scryfall_objects/ruling/test/ruling.json").unwrap();
    let tokens = parse_json_string(s).unwrap();
    let res = Ruling::deserialize(tokens).unwrap();

    let should = Ruling {
        oracle_id: crate::UUID::try_from("afa49a09-146f-4439-850e-dd1938c93cef").unwrap(),
        source: String::from("scryfall"),
        published_at: String::from("2015-01-19"),
        comment: String::from("Derevi, Empyrial Tactician is banned as a commander in Duel Commander format, but it may be part of your deck."),
    };

    assert_eq!(res, should);
}