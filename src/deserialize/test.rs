use super::{
    ParseError,
    DesValue,
    read_string_in_quotes,
    read_field_name,
    read_string_out_quotes,
    read_field,
    read_array,
    read_object,
    parse_json_string,
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
fn read_string_out_quotes_1() {
    let s = "false";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from("false"), res);
}

#[test]
fn read_string_out_quotes_2() {
    let s = "false,";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from("false"), res);
}

#[test]
fn read_string_out_quotes_3() {
    let s = "   123.5   ";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from("123.5"), res);
}

#[test]
fn read_string_out_quotes_4() {
    let s = "";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from(""), res);
}

#[test]
fn read_string_out_quotes_5() {
    let s = "                     ";

    let mut i = 0;
    let res = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from(""), res);
}

#[test]
fn read_string_out_quotes_6() {
    let s = "   abc   def   ";

    let mut i = 0;
    let res1 = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from("abc"), res1);
    let res2 = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from("def"), res2);
}

#[test]
fn read_string_out_quotes_7() {
    let s = "abc, def";

    let mut i = 0;
    let res1 = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from("abc"), res1);
    
    assert_eq!(",", &s[i..i+1]);

    i += 1;
    let res2 = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from("def"), res2);
}

#[test]
fn read_string_out_quotes_8() {
    let s = "    ";

    let mut i = 0;
    let res1 = read_string_out_quotes(&s, &mut i);
    assert_eq!(String::from(""), res1);
    
    let mut chars = s[i..].char_indices();
    assert_eq!(chars.next(), None);
}

#[test]
fn read_field_string() {
    let s = "\"FieldName\": \"Hello, World!\"";
    let mut i = 0;
    let res = read_field(s, &mut i).unwrap();

    assert_eq!(
        res,
        (String::from("FieldName"), DesValue::String(String::from("Hello, World!")))
    )
}

#[test]
fn read_field_bool_true() {
    let s = "\"FieldName\": true";
    let mut i = 0;
    let res = read_field(s, &mut i).unwrap();

    assert_eq!(
        res,
        (String::from("FieldName"), DesValue::Bool(true))
    )
}

#[test]
fn read_field_bool_false() {
    let s = "\"FieldName\": false";
    let mut i = 0;
    let res = read_field(s, &mut i).unwrap();

    assert_eq!(
        res,
        (String::from("FieldName"), DesValue::Bool(false))
    )
}

#[test]
fn read_field_num1() {
    let s = "\"FieldName\": 123.4";
    let mut i = 0;
    let res = read_field(s, &mut i).unwrap();

    assert_eq!(
        res,
        (String::from("FieldName"), DesValue::from(123.4))
    )
}

#[test]
fn read_field_num2() {
    let s = "\"FieldName\": 1234";
    let mut i = 0;
    let res = read_field(s, &mut i).unwrap();

    assert_eq!(
        res,
        (String::from("FieldName"), DesValue::from(1234))
    )
}

#[test]
fn read_field_num3() {
    let s = "\"FieldName\": 12.3.4";
    let mut i = 0;
    let res = read_field(s, &mut i);

    assert_eq!(
        res,
        Err(ParseError::UnexpectedToken('.'))
    )
}

#[test]
fn read_arr_1() {
    let s = "[ 1234 ]";
    let mut i = 0;
    let res = read_array(s, &mut i).unwrap();

    assert_eq!(
        res,
        DesValue::Array(Box::from([DesValue::from(1234)]))
    )
}

#[test]
fn read_arr_2() {
    let s = "[ 1234, 4321 ]";
    let mut i = 0;
    let res = read_array(s, &mut i).unwrap();

    assert_eq!(
        res,
        DesValue::Array(Box::from([
            DesValue::from(1234),
            DesValue::from(4321)
        ]))
    )
}

#[test]
fn read_arr_3() {
    let s = "\"ArrayField\": [ 1234, 4321 ]";
    let mut i = 0;
    let res = read_field(s, &mut i).unwrap();

    assert_eq!(
        res,
        (
        String::from("ArrayField"),
        DesValue::Array(Box::from([
            DesValue::from(1234),
            DesValue::from(4321)
        ])))
    )
}

#[test]
fn read_arr_4() {
    let s = "\"ArrayField\": [ [], [1234, 4321] ]";
    let mut i = 0;
    let res = read_field(s, &mut i).unwrap();

    let arr1 = DesValue::Array(Box::from([]));
    let arr2 = DesValue::Array(Box::from([
        DesValue::from(1234),
        DesValue::from(4321)
    ]));

    assert_eq!(
        res,
        (
        String::from("ArrayField"),
        DesValue::Array(Box::from([
            arr1,
            arr2
        ])))
    )
}

#[test]
fn read_arr_5() {
    let s = "\"ArrayField\": [ [], [1234, 4321] ";
    let mut i = 0;
    let res = read_field(s, &mut i);

    assert_eq!(
        res,
        Err(ParseError::ExpectedToken(']'))
    )
}

#[test]
fn read_arr_6() {
    let s = "\"ArrayField\": [ [, [1234, 4321] ]";
    let mut i = 0;
    let res = read_field(s, &mut i);

    assert_eq!(
        res,
        Err(ParseError::UnexpectedToken(','))
    )
}

#[test]
fn read_null_1() {
    let s = "{
        \"NullField\": Null
    }";
    let mut i = 0;
    let res = read_object(s, &mut i).unwrap();

    assert_eq!(
        res,
        DesValue::Object(Box::from([
            (String::from("NullField"), DesValue::Null)
        ]))
    )
}

#[test]
fn read_object_1() {
    let s = "{}";
    let mut i = 0;
    let res = read_object(s, &mut i).unwrap();

    assert_eq!(
        res,
        DesValue::Object(Box::from([]))
    )
}

#[test]
fn read_object_2() {
    let s = "{
        \"FieldName\": 1234
    }";

    let mut i = 0;
    let res = read_object(s, &mut i).unwrap();

    assert_eq!(
        res,
        DesValue::Object(Box::from([
            (String::from("FieldName"), DesValue::from(1234))
        ]))
    )
}

#[test]
fn read_object_3() {
    let s = "{
        \"FieldName\": 1234,
        \"ArrayField\": [ 1234, 4321 ]
    }";
    
    let mut i = 0;
    let res = read_object(s, &mut i).unwrap();

    let arr_field = DesValue::Array(Box::from([
        DesValue::from(1234), 
        DesValue::from(4321)
        ]));

    assert_eq!(
        res,
        DesValue::Object(Box::from([
            (String::from("FieldName"), DesValue::from(1234)),
            (String::from("ArrayField"), arr_field)
        ]))
    )
}

#[test]
fn read_object_4() {
    let s = std::fs::read_to_string("src/deserialize/big_test.json").unwrap();
    println!("{}", &s[5691..]);
    let res = parse_json_string(s);
    let dbg = format!("{:#?}", res);
    std::fs::write("src/deserialize/output.txt", &dbg.as_bytes()).unwrap();
    assert!(res.is_ok())
}