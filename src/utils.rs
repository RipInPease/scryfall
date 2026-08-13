/// Converts a hex char to decimal
pub fn hex_digit_to_dec(c: char) -> Result<u8, ()> {
    match u8::try_from(c) {
        Ok(v @ b'0'..=b'9') => {
            Ok(v - b'0')
        },
        Ok(v @ b'a'..=b'f') => {
            Ok(v - b'a' + 10)
        },
        Ok(v @ b'A'..=b'F') => {
            Ok(v - b'A' + 10)
        },
        _ => Err(())
    }
}