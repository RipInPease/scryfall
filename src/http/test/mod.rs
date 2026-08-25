use super::{
    Response, 
    HttpStatus, 
    query_string_to_http, 
    Connection, 
    RestRequest
};

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

#[test]
fn read_status_1() {
    let mut file = std::fs::File::open("src/http/test/headers.txt").unwrap();
    let http_status = Response::read_version_status(&mut file).unwrap();
    
    let should = HttpStatus {
        code: 200,
        message: "OK".to_string()
    };

    assert_eq!(http_status, should);
}

#[test]
fn read_headers_1() {
    use std::collections::HashMap;

    let mut file = std::fs::File::open("src/http/test/headers.txt").unwrap();
    let _ = Response::read_version_status(&mut file).unwrap();

    let headers = Response::read_headers(&mut file).unwrap();
    
    let should: HashMap<String, String> = HashMap::from([
        ("Date".to_string(), "Tue, 28 Jul 2026 06:25:38 GMT".to_string()),
        ("Content-Type".to_string(), "application/json; charset=utf-8".to_string()),
        ("Transfer-Encoding".to_string(), "chunked".to_string()),
        ("Connection".to_string(), "close".to_string()),
        ("access-control-allow-headers".to_string(), "Accept, Accept-Charset, Accept-Language, Authorization, Cache-Control, Content-Language, Content-Type, DNT, Host, If-Modified-Since, Keep-Alive, Origin, Referer, User-Agent, X-Requested-With".to_string()),
        ("access-control-allow-methods".to_string(), "GET, POST, DELETE, OPTIONS".to_string()),
        ("access-control-allow-origin".to_string(), "*".to_string()),
        ("access-control-max-age".to_string(), "300".to_string()),
        ("Cache-Control".to_string(), "public, max-age=57600".to_string()),
    ]);

    assert_eq!(headers, should);
}

#[test]
fn read_data_1() {
    let mut file = std::fs::File::open("src/http/test/headers.txt").unwrap();
    Response::read_version_status(&mut file).unwrap();
    Response::read_headers(&mut file).unwrap();
    let data = Response::read_data(&mut file).unwrap();
    
    assert_eq!(&(*data), b"abcdefghijklmnopq");
}

#[test]
fn query_string_to_http_1() {
    let s = "this is a query";
    let res = query_string_to_http(&s);

    assert_eq!(res, "%74%68%69%73%20%69%73%20%61%20%71%75%65%72%79");
}

#[test]
fn rest_request_1() {
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open("src/http/test/files_used_inside_tests/rest_request_1.txt")
        .unwrap();

    let mut connection = Connection::new(
        file, 
        [("Header-Title".to_string(), "Header-Value".to_string())]
    );

    let request = RestRequest::GET { 
        path: "/path/to/get".to_string(), 
        parameters: Box::new([
            ("Parameter-Name".to_string(), "Paramater-Value".to_string())
        ])
    };

    connection.send_rest_request(request).unwrap();
}

#[test]
fn rest_request_2() {
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open("src/http/test/files_used_inside_tests/rest_request_2.txt")
        .unwrap();

    let mut connection = Connection::new(
        file, 
        [("Header-Title".to_string(), "Header-Value".to_string())]
    );

    let request = RestRequest::POST { 
        path: "path/to/post".to_string(), 
        data: Box::new(b"This is some data I guess\nAnd this is a new line".to_owned())
    };

    connection.send_rest_request(request).unwrap();
}

#[test]
fn rest_request_3() {
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .open("src/http/test/files_used_inside_tests/rest_request_3.txt")
        .unwrap();

    let mut connection = Connection::new(
        file, 
        [
            ("Header-Title".to_string(), "Header-Value".to_string()),
            ("Transfer-Encoding".to_string(), "Chunked".to_string())
        ]
    );

    let request = RestRequest::POST { 
        path: "path/to/post".to_string(), 
        data: Box::new(b"This is some data I guess\nAnd this is a new line".to_owned())
    };

    connection.send_rest_request(request).unwrap();
}