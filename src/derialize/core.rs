use crate::*;

use std::{fs::read, io::Read};

impl Derialize for UUID {
    fn derialize<R: Read>(r: &mut R) -> Result<Self, DerializeError>
    where Self: Sized
    {
        let mut buf = [0; 1];
        
        while buf[0] != b'\"' {
            r.read(&mut buf)?;
        }

        let mut buf = [0;36];
        

        Err(DerializeError::UEO)
    }
}