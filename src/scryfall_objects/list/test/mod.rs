use super::*;
use crate::deserialize::{Deserialize, parse_json_string};

#[test]
fn deserialize_list_cards() {
    let s = std::fs::read_to_string("src/scryfall_objects/list/test/list_cards.json").unwrap();
    let tokens = parse_json_string(s).unwrap();
    let list = List::deserialize(tokens).unwrap();

    for entry in list.data {
        assert!(entry.is_card())
    }
}

#[test]
fn deserialize_list_rulings() {
    let s = std::fs::read_to_string("src/scryfall_objects/list/test/list_rulings.json").unwrap();
    let tokens = parse_json_string(s).unwrap();
    let list = List::deserialize(tokens).unwrap();
    
    for entry in list.data {
        assert!(entry.is_ruling())
    }
}

#[test]
fn deserialize_list_card_symbols() {
    let s = std::fs::read_to_string("src/scryfall_objects/list/test/list_card_symbols.json").unwrap();
    let tokens = parse_json_string(s).unwrap();
    let list = List::deserialize(tokens).unwrap();
    
    for entry in list.data {
        assert!(entry.is_card_symbol())
    }
}