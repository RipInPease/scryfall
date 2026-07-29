use super::*;

#[test]
fn read_char_1() {
    let mut file = std::fs::File::open("src/http/test/text.txt").unwrap();

    let mut s = String::new();
    for _ in 0..13 {
        let c = Response::read_char(&mut file).unwrap();
        s.push(c);
    }

    assert_eq!(&s[..], "Hello, World!");
    
    
    let mut s = String::new();
    for _ in 0..10 {
        let c = Response::read_char(&mut file).unwrap();
        println!("Char: {c}");
        s.push(c);
    }
    
    assert_eq!(&s[..], "Aéñø€中✓😀🚀𝄞");
}

#[test]
fn read_line_1() {
    let mut file = std::fs::File::open("src/http/test/text.txt").unwrap();
    let s = Response::read_line(&mut file).unwrap();
    assert_eq!(&s[..], "Hello, World!Aéñø€中✓😀🚀𝄞");

    let s = Response::read_line(&mut file).unwrap();
    assert_eq!(&s[..], "Line2");

    let s = Response::read_line(&mut file).unwrap();
    assert_eq!(&s[..], "Line3");

    let s = Response::read_line(&mut file).unwrap();
    assert_eq!(&s[..], "");
}