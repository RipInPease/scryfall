use super::Catalog;
use crate::{deserialize::{Deserialize, parse_json_string}};
use crate::scryfall_objects::*;

#[test]
fn deserialize_catalog_1() {
    let s = std::fs::read_to_string("src/scryfall_objects/catalog/test/catalog.json").unwrap();
    let tokens = parse_json_string(s).unwrap();
    let res = Catalog::deserialize(tokens).unwrap();

    let should = Catalog {
        uri: URI(String::from("https://api.scryfall.com/catalog/land-types")),
        total_values: 18,
        data: vec![
            String::from("Cave",),
            String::from("Cloud",),
            String::from("Desert",),
            String::from("Forest",),
            String::from("Gate",),
            String::from("Island",),
            String::from("Lair",),
            String::from("Locus",),
            String::from("Mine",),
            String::from("Mountain",),
            String::from("Sphere",),
            String::from("Plains",),
            String::from("Planet",),
            String::from("Power-Plant",),
            String::from("Swamp",),
            String::from("Tower",),
            String::from("Town",),
            String::from("Urza's"),
        ].into_boxed_slice()
    };

    assert_eq!(res, should);
}