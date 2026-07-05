use crate::*;

#[test]
fn uuid_from_ascii() {
    let mut b = [b'F';36];
    b[0] = b'F';
    b[1] = b'F';

    let uuid: UUID = b.try_into().unwrap();
    assert_eq!(uuid.0[0], 255);
}