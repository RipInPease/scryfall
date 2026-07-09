use crate::*;
use crate::derialize::utils;

use std::io::Read;

impl Deserialize for String {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DeserializeError>
    where Self: Sized
    {
        utils::read_to_quote(r)?;

        let mut s = String::new();

        let mut buf = [0];
        r.read_exact(&mut buf)?;

        while buf[0] != b'\"' {
            s.push(buf[0] as char);
            r.read_exact(&mut buf)?;
        }

        Ok(s)
    }
}

impl Deserialize for u16 {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DeserializeError>
    where Self: Sized
    {
        let first_char = utils::read_to_non_whitespace(r)?;
        if !first_char.is_ascii_digit() {
            return Err(DeserializeError::ParsingError)
        }

        let mut res = (first_char - b'0') as u16;
        
        let mut buf = [0];
        r.read_exact(&mut buf)?;
        while buf[0].is_ascii_digit() {
            res *= 10;
            res += (buf[0] - b'0') as u16;
            r.read_exact(&mut buf)?;
        }

        Ok(res)
    }
}

impl Deserialize for UUID {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DeserializeError>
    where Self: Sized
    {
        // Read until the start of the UUID which is inside quotes
        utils::read_to_quote(r)?;

        let mut buf = [0;36];
        r.read_exact(&mut buf)?;

        let uuid = match UUID::try_from(buf) {
            Ok(v) => v,
            Err(_) => return Err(DeserializeError::ParsingError)
        };

        let mut buf = [0; 1];
        r.read(&mut buf)?;
        if buf[0] != b'\"' {
            return Err(DeserializeError::ExpectedToken('\"'));
        }

        Ok(uuid)
    }
}