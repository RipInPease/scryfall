use super::Error;
use crate::deserialize::{Deserialize, parse_json_string};

#[test]
fn deserialize_error_object() {
    let s = std::fs::read_to_string("src/scryfall_objects/error/test/error.json").unwrap();
    let tokens = parse_json_string(s).unwrap();
    let res = Error::deserialize(tokens).unwrap();

    let should = Error {
        status: 400,
        code: String::from("bad_request"),
        details: String::from("All of your terms were ignored."),
        warnings: Some(vec![
            String::from("Invalid expression “is:slick” was ignored. Checking if cards are “slick” is not supported"),
            String::from("Invalid expression “cmc>cmc” was ignored. The sides of your comparison must be different."),
        ].into_boxed_slice()),
        error_type: None
    };

    assert_eq!(res, should);
}