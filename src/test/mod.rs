use std::fs::File;

use crate::*;

#[test]
fn uuid_from_file1() {
    use std::fs::File;
    let mut f = File::open("src/test/uuid1.json").unwrap();

    let uuid_from_file = UUID::derialize(&mut f).unwrap();
    let should_be = UUID(
    [
        241,
        29,
        115,
        17,
        64,
        102,
        74,
        93,
        186,
        40,
        152,
        87,
        250,
        112,
        122,
        11,
    ]);

    assert_eq!(uuid_from_file, should_be);
}


#[test]
fn string_from_file() {
    let mut f = File::open("src/test/string.json").unwrap();
    let s = String::derialize(&mut f).unwrap();

    assert_eq!(&s, "Hello, World!");
}