use crate::deserialize::*;
use crate::scryfall_objects::*;

#[test]
fn uuid_1() {
    let val = DesValue::String(String::from("f11d7311-4066-4a5d-ba28-9857fa707a0b"));
    let res = UUID::deserialize(val);

    assert!(res.is_ok())
}