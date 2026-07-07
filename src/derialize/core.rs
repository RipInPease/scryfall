use crate::*;

use std::{fs::read, io::Read};

impl Derialize for UUID {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DerializeError>
    where Self: Sized
    {
        // Read until the start of the UUID which is inside quotes
        let mut buf = [0; 1];
        while buf[0] != b'\"' {
            r.read(&mut buf)?;
        }

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