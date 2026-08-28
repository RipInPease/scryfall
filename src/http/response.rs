use std::collections::HashMap;
use std::io::Read;

use super::Error;

/// Only for HTTP 1.1
#[derive(Debug, PartialEq)]
pub struct Response {
    pub status_code: HttpStatus,
    pub headers: HashMap<String, String>,
    pub data: Box<[u8]>
}

#[derive(Debug, PartialEq)]
pub struct HttpStatus {
    pub code: u16,
    pub message: String
}

impl HttpStatus {
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.code)
    }

    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.code)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.code)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.code)
    }
}

impl Response {
    /// Tries to read a [`char`] from a reader.
    /// If data was read, but not a valid char returns [`Error::NonUTF8`].
    /// If no data was read returns [`IOErrorKind::UnexpectedEof`]
    pub fn read_char<R: Read>(r: &mut R) -> Result<char, Error> {
        let mut buf = [0;4];
        r.read_exact(&mut buf[0..1])?;

        let utf_8_len = match buf[0] {
            0b0000_0000..=0b0111_1111 => 1,
            0b1100_0000..=0b1101_1111 => 2,
            0b1110_0000..=0b1110_1111 => 3,
            0b1111_0000..=0b1111_0111 => 4,
            _ =>  return Err(Error::NonUTF8)
        };

        if utf_8_len == 1 {
            return Ok(buf[0].into())
        } else {
            if r.read_exact(&mut buf[1..utf_8_len]).is_err() {
                return Err(Error::NonUTF8)
            }
        }

        match str::from_utf8(&buf[0..utf_8_len]) {
            Ok(s) => Ok(s.chars().next().unwrap()),
            Err(_) => Err(Error::NonUTF8)
        }
    }


    /// Tries to read a line of [`String`] from a reader.
    /// A line is any piece of text seperated by a newline
    /// 
    /// 
    /// If data was read, but not a valid char returns [`Error::NonUTF8`].
    /// If no data was read returns an empty string
    pub (crate) fn read_line<R: Read>(r: &mut R) -> Result<String, Error> {
        let mut s = String::new();

        let mut carriage = false;
        loop {
            let c = match Self::read_char(r) {
                Ok(c) => c,
                Err(e) => {
                    if e.is_io_error() {
                        return Ok(s)
                    } else {
                        return Err(e)
                    }
                }
            };

            if c == '\r' && !carriage {
                carriage = true;
            } else if c == '\n' {
                return Ok(s)
            } else if carriage {
                s.push('\r');
                s.push(c);
                carriage = false;
            } else {
                s.push(c);
            }
        }
    }

    /// Reads a response from [`TcpStream`]
    pub (crate) fn read_from_stream<R: Read>(stream: &mut R) -> Result<Self, Error>  {
        let status_code = Self::read_version_status(stream)?;
        let headers = Self::read_headers(stream)?;
        let data = Self::read_data(stream)?;

        Ok(Self {
            status_code,
            headers,
            data
        })
    }

    /// Reads HTTP version and status
    pub (crate) fn read_version_status<R: Read>(stream: &mut R) -> Result<HttpStatus, Error> {
        let line = Self::read_line(stream)?;

        let mut fields = line.split(" ");
        let version = fields.next().ok_or(Error::ProtocolDeviation)?;
        let code = fields.next().ok_or(Error::ProtocolDeviation)?;
        let code = code.parse::<u16>().map_err(|_| Error::ProtocolDeviation)?;
        let mut message = fields.next().ok_or(Error::ProtocolDeviation)?.to_string();

        for s in fields {
            message += s;
        }

        if version != "HTTP/1.1" {
            return Err(Error::IncorrectValue { 
                expected: "HTTP/1.1", 
                got: version.to_string() 
            })
        } else if !(100..600).contains(&code) {
            return Err(Error::ProtocolDeviation)
        }

        Ok(HttpStatus {
            code,
            message
        })
    }

    /// Reads HTTP headers
    pub(crate) fn read_headers<R: Read>(stream: &mut R) -> Result<HashMap<String, String>, Error>  {
        let mut headers = HashMap::new();
        
        while let line = Self::read_line(stream)? && line.len() > 0 {
            let (header, val) = line
                .split_once(":")
                .ok_or(Error::ProtocolDeviation)?;

            let header = header.to_string();
            let val = val.trim().to_string();

            headers.insert(header, val);
        }

        Ok(headers)
    }

    /// Reads the data portion of HTTP response
    pub (crate) fn read_data<R: Read>(stream: &mut R) -> Result<Box<[u8]>, Error> {
        let mut data: Vec<u8> = Vec::new();
        while let line = Self::read_line(stream)? && line.len() > 0 {
            let chunk_size = Self::hex_to_dec(line).map_err(|_| Error::ProtocolDeviation)?;
            let mut chunk =  vec![0; chunk_size];
            
            stream.read_exact(&mut chunk)?;
            data.extend(chunk.into_iter());

            // This read_line is to read away the potential new line char
            Self::read_line(stream)?;
            
            if chunk_size == 0 {
                break;
            }
        }

        Ok(data.into_boxed_slice())
    }

    /// Converts a hex [`String`] to decimal [`usize`]
    /// 
    /// # Panics
    /// 
    /// Panics if the string to parse is larger than [`usize`]
    pub (crate) fn hex_to_dec(s: String) -> Result<usize, ()> {
        let mut res: usize = 0;

        for c in s.chars() {
            if !c.is_digit(16) {
                return Err(());
            }

            let digit = crate::utils::hex_digit_to_dec(c)?;
            res *= 16;
            res += digit as usize;
        }

        Ok(res)
    }
}
