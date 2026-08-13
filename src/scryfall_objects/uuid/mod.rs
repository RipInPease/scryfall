use crate::{Deserialize, DesValue, ParseError};

#[derive(Debug, Clone, PartialEq)]
/// Universal unique identifier
pub struct UUID(pub [u8;16]);

impl UUID {
    /// Turns the ascii 1-A to numerical 1-16
    fn ascii_to_hex(val: u8) -> Result<u8, ()> {
        // ascii value for digits 0-9
        if val >= b'0' && val <= b'9' {
            Ok(val - b'0')
        } 
        // ascii value for digits A-F
        else if val >= b'A' && val <= b'F' {
            Ok(val - b'A' + 10)
        }
        // ascii value for digits a-f
        else if val >= b'a' && val <= b'f' {
            Ok(val - b'a' + 10)
        } else {
            Err(())
        }
    }
}

/// Where this is an array of 36 chars
impl TryFrom<&str> for UUID {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 36 {
            return Err(())
        }

        let value = value.as_bytes();
        let mut bytes = [0;16];
        let mut i = 0;  // Index in bytes
        let mut j = 0;  // Number of chars processed

        let mut v1 = 0; // The first char in the byte
        let mut v2; // The second char in the byte

        for b in value {
            // Skip the dashes
            if *b == b'-' {
                continue
            }

            if i >= value.len() {
                return Err(())
            }

            if j == 0 {
                v1 = UUID::ascii_to_hex(*b)?;
                j += 1;
            } else if j == 1 {
                v2 = UUID::ascii_to_hex(*b)?;
                bytes[i] = v1 * 16 + v2;
                i += 1;
                j = 0;
            }
        }
        
        Ok(Self(bytes))
    }
}

impl Deserialize for UUID {
    fn deserialize(tokens: DesValue) -> Result<Self, ParseError>
    where Self: Sized
    {
        match tokens {
            DesValue::String(s) => {
                if s.len() != 36 {
                    Err(ParseError::UnkownVal(s))
                } else {
                    let uuid = UUID::try_from(&s[..]);
                    
                    if uuid.is_err() {
                        Err(ParseError::UnkownVal(s))
                    } else {
                        let uuid = uuid.unwrap();
                        Ok(uuid)
                    }
                }
            },
            _ => Err(ParseError::MismatchedType)
        }
    }
}
