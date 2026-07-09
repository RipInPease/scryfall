use std::io::Read;

use crate::DeserializeError;

/// Read until a quote has been read
/// 
/// The quote will not be included in any
/// subsequent reads
pub(crate) fn read_to_quote<R: Read>(r: &mut R) -> Result<(), DeserializeError> {
    let mut buf = [0; 1];
    while buf[0] != b'\"' {
        r.read(&mut buf)?;
    }

    Ok(())
}

/// Read until a non-whitespace has been read
/// 
/// If successful, returns the first value that is not whitespace.
/// All subsequent reads will be from the seconds value after whitespace has ended
pub(crate) fn read_to_non_whitespace<R: Read>(r: &mut R) -> Result<u8, DeserializeError> {
    let mut buf: [u8; 1] = [0; 1];
    while buf[0].is_ascii_whitespace() {
        r.read(&mut buf)?;
    }

    Ok(buf[0])
}