#[cfg(test)]
mod test;

pub mod card;

/// Derialize all types here
pub(crate) mod derialize;


#[derive(Debug, Clone, PartialEq)]
pub struct URI(String);


#[derive(Debug, Clone, PartialEq)]
/// Universal unique identifier
pub struct UUID(pub [u8;16]);

impl UUID {
    /// Turns the ascii 1-A to numerical 1-16
    fn ascii_to_hex(val: u8) -> Result<u8, ()> {
        // ascii value for digits 0-9
        if val >= b'0' && val <= b'9' {
            Ok(val - 48)
        } 
        // ascii value for digits A-F
        else if val >= b'A' && val <= b'F' {
            Ok(val - 55)
        }
        // ascii value for digits a-f
        else if val >= b'a' && val <= b'f' {
            Ok(val - 87)
        } else {
            Err(())
        }
    }
}

/// Where this is an array of 36 chars
impl TryFrom<[u8; 36]> for UUID {
    type Error = ();

    fn try_from(value: [u8; 36]) -> Result<Self, Self::Error> {
        let mut bytes = [0;16];
        let mut i = 0;  // Index in bytes
        let mut j = 0;  // Number of chars processed

        let mut v1 = 0; // The first char in the byte
        let mut v2; // The second char in the byte

        for b in value {
            // Skip the dashes
            if b == b'-' {
                continue
            }

            if i >= value.len() {
                return Err(())
            }

            if j == 0 {
                v1 = UUID::ascii_to_hex(b)?;
                j += 1;
            } else if j == 1 {
                v2 = UUID::ascii_to_hex(b)?;
                bytes[i] = v1 * 16 + v2;
                i += 1;
                j = 0;
            }
        }

        Ok(Self(bytes))
    }
}


pub struct List {
    pub has_more    : bool,
    pub data        : Vec<Data>,
    pub next_page   : Option<URI>,

    /// If this is a list of card object this field will 
    /// be [`Some`]
    pub total_cards : Option<i32>,

    /// Human readable warning returned to your request
    pub warnings    : Option<String>
}

/// The different kinds of data a [`List`] can contain
pub enum Data {
    Cards(card::Card),
}
