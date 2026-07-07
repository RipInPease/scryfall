use crate::*;
use crate::derialize::utils;

use std::io::Read;

impl Derialize for String {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DerializeError>
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

impl Derialize for UUID {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DerializeError>
    where Self: Sized
    {
        // Read until the start of the UUID which is inside quotes
        utils::read_to_quote(r)?;

        let mut buf = [0;36];
        r.read_exact(&mut buf)?;

        let uuid = match UUID::try_from(buf) {
            Ok(v) => v,
            Err(_) => return Err(DerializeError::ParsingError)
        };

        let mut buf = [0; 1];
        r.read(&mut buf)?;
        if buf[0] != b'\"' {
            return Err(DerializeError::ExpectedToken('\"'));
        }

        Ok(uuid)
    }
}