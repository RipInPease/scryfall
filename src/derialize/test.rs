
use super::{
    ParseError,
    read_string_in_quotes,
    read_field_name
};

#[test]
fn read_string_success1() {
    let s1 = "Hello, World!\"";

    let mut i = 0;
    let s2 = read_string_in_quotes(&s1, &mut i);
    assert_eq!("Hello, World!", &s2.unwrap());
}

#[test]
fn read_string_success2() {
    let s1 = "\"Hello, World!\"";

    let mut i = 1;
    let s2 = read_string_in_quotes(&s1, &mut i);
    assert_eq!("Hello, World!", &s2.unwrap());
}

#[test]
fn read_string_success3() {
    let s1 = "\"Hello, World!\"Hello";

    let mut i = 1;
    let _ = read_string_in_quotes(&s1, &mut i);
    assert_eq!(&s1[i..], "\"Hello");
}

#[test]
fn read_string_error() {
    let s1 = "Hello, World!";

    let mut i = 0;
    let s2 = read_string_in_quotes(&s1, &mut i);
    assert_eq!(Err(ParseError::ExpectedToken('\"')), s2);
}

#[test]
fn read_field_name_success1() {
    let s1 = "\"Hello, World!\"";

    let mut i = 0;
    let s2 = read_field_name(&s1, &mut i);
    assert_eq!("Hello, World!", s2.unwrap());
}

#[test]
fn read_field_name_error() {
    let s1 = "\"Hello, World!";

    let mut i = 0;
    let s2 = read_field_name(&s1, &mut i);
    assert_eq!(Err(ParseError::ExpectedToken('\"')), s2);
}

#[test]
fn read_field_name_success2() {
    let s = "\"Name1\", \"Name2\"";

    let mut i = 0;
    let name1 = read_field_name(&s, &mut i);
    let name2 = read_field_name(&s, &mut i);
    assert_eq!("Name1", name1.unwrap());
    assert_eq!("Name2", name2.unwrap());
}

#[test]
fn read_field_name_success3() {
    let s = "\"Name1\", \"Name2\"";

    let mut i = 0;
    let _ = read_field_name(&s, &mut i);
    assert_eq!(", \"Name2\"", &s[i..]);
}