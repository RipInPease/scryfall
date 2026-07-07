use std::io::Read;

use crate::DerializeError;

/// Read until a quote has been read
/// 
/// The quote will not be included in any
/// subsequent reads
pub(crate) fn read_to_quote<R: Read>(r: &mut R) -> Result<(), DerializeError> {
    let mut buf = [0; 1];
    while buf[0] != b'\"' {
        r.read(&mut buf)?;
    }

    Ok(())
}