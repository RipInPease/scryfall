use super::{
    ParseError,
    read_string_in_quotes,
    read_field_name,
    read_string_out_quotes
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
    assert_eq!(&s1[i..], "Hello");
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
    assert_eq!(&s[i..i+1], ",");
    i += 1;
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

#[test]
fn read_string_out_quotes_success1() {
    let s = "false";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("false")), res);
}

#[test]
fn read_string_out_quotes_success2() {
    let s = "false,";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("false")), res);
}

#[test]
fn read_string_out_quotes_success3() {
    let s = "   123.5   ";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("123.5")), res);
}

#[test]
fn read_string_out_quotes_success4() {
    let s = "";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("")), res);
}

#[test]
fn read_string_out_quotes_success5() {
    let s = "                     ";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("")), res);
}

#[test]
fn read_string_out_quotes_success6() {
    let s = "   abc   def   ";

    let mut i = 0;
    let res1 = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("abc")), res1);
    let res2 = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("def")), res2);
}

#[test]
fn read_string_out_quotes_success7() {
    let s = "abc, def";

    let mut i = 0;
    let res1 = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("abc")), res1);
    
    assert_eq!(",", &s[i..i+1]);

    i += 1;
    let res2 = read_string_out_quotes(&s, &mut i);
    assert_eq!(Ok(String::from("def")), res2);
}