use std::io::{Read, Write, Result as IOResult};
use std::collections::HashMap;

use super::{Error, Response, query_string_to_http};

/// A connection used to send REST API requests and read responses from 
/// using HTTP 1.1 
#[derive(Debug, PartialEq)]
pub struct Connection<T: Read + Write> {
    stream: T,
    headers: HashMap<String, String>
}

impl<T: Read + Write> Read for Connection<T> {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        self.stream.read(buf)
    }
}

impl<T: Read + Write> Write for Connection<T> {
   fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
       self.stream.write(buf)
   }

   fn flush(&mut self) -> IOResult<()> {
       self.stream.flush()
   }
}

impl<T: Read + Write> Connection<T> {
    /// Creates a new [`Connection`] with given stream and headers
    pub fn new<H>(stream: T, headers: H) -> Self
    where
        H: Into<HashMap<String, String>>
    {
        Self {
            stream,
            headers: headers.into()
        }
    }

    /// Sends a REST API request
    pub fn send_rest_request(&mut self, request: RestRequest) -> Result<(), Error> {
        let mut data = String::with_capacity(8192);

        // Base_request_string is for example GET /path/to/get
        let (base_request_string, request_data) = request.string_and_data();
        data.push_str(&base_request_string);
        data.push_str(" HTTP/1.1\r\n");

        let request_data = match self.header_transfer_type() {
            TransferEncoding::Chunked => {
                Self::data_to_chunked(request_data, 8192)
            },
            TransferEncoding::ContentLength => {
                self.headers.insert(
                    "Content-Length".to_string(), 
                    request_data.len().to_string()
                );

                request_data
            }
        };

        let header_string = Self::headers_to_string(&self.headers);
        data.push_str(&header_string);

        let mut data = data.into_bytes();
        data.extend(request_data);

        self.write_all(&data)?;

        Ok(())
    }

    /// Checks the transfer method. If no transfer method was set, 
    /// defaults to [`TransferEncoding::ContentLength`].
    /// 
    /// If [`TransferEncoding::Chunked`] is set as well as some others by mistake,
    /// removes all other conflincting headers
    fn header_transfer_type(&mut self) -> TransferEncoding {
        if self.headers.contains_key("Transfer-Encoding") {
            self.headers.remove("Content-Length");
            TransferEncoding::Chunked
        } else {
            self.headers.insert("Content-Length".to_string(), "0".to_string());
            TransferEncoding::ContentLength
        }
    }

    /// Turns an array of headers into a single String
    fn headers_to_string(headers: &HashMap<String, String>) -> String {
        let mut res = String::with_capacity(headers.len() * 32 + 100);

        for header in headers {
            res.push_str(&header.0);
            res.push_str(": ");
            res.push_str(&header.1);
            res.push_str("\r\n");
        }

        res.push_str("\r\n");
        res
    }

    /// Turns a data string to chunked data string
    /// 
    /// # Panics
    /// 
    /// Panics if `chunk_size` is 0
    fn data_to_chunked(data: Box<[u8]>, chunk_size: usize) ->Box<[u8]> {
        let mut res: Vec<u8> = Vec::with_capacity(
            data.len() + data.len() / chunk_size + 100
        );
    
        for chunk in data.chunks(chunk_size) {
            let current_chunk_size = chunk.len();
            let size_formatted = format!("{:0x}", current_chunk_size).into_bytes();

            res.extend(size_formatted);
            res.extend(b"\r\n");
            res.extend(chunk);
            res.extend(b"\r\n");
        }

        res.extend(b"0\r\n\r\n");

        res.into_boxed_slice()
    }

    /// Reads a [`Response`] from the connection
    pub fn read_response(&mut self) -> Result<Response, Error> {
        Response::read_from_stream(&mut self.stream)
    }
}

#[derive(Debug, PartialEq)]
pub enum RestRequest {
    /// Retreive data
    GET {
        /// E.G. /users/123
        path: String,

        /// [ParameterName, Parameter Value]
        parameters: Box<[(String, String)]>,
    },

    /// Creates a new resource
    POST {
        path: String,
        data: Box<[u8]>
    },

    /// Update or create a resource
    PUT {
        path: String,
        data: Box<[u8]>,
    },

    /// Partially update a resource
    PATCH {
        path: String,
        data: Box<[u8]>,
    },

    DELETE {
        path: String
    }
}

impl RestRequest {
    /// Returns a string of how the REST API request would look
    /// and eturns the data associated with the request. 
    /// If there was no associated data, returns an empty array
    pub (crate) fn string_and_data(self) -> (String, Box<[u8]>) {
        let mut res = String::with_capacity(512);
        let mut res_data: Box<[u8]> = Box::new([]);

        match self {
            Self::DELETE { path } => {
                res.push_str("DELETE ");
                res.push_str(&path);
            },
            Self::GET { path, parameters } => {
                res.push_str("GET ");
                res.push_str(&path);

                if parameters.len() > 0 {
                    res.push('?');
                    res.push_str(&Self::parameters_to_string(parameters));
                }
            },
            Self::PATCH { path, data } => {
                res.push_str("PATCH ");
                res.push_str(&path);
                res_data = data
            },
            Self::POST { path, data } => {
                res.push_str("POST ");
                res.push_str(&path);
                res_data = data
            },
            Self::PUT { path, data } => {
                res.push_str("PUT ");
                res.push_str(&path);
                res_data = data
            }
        }

        (res, res_data)
    }

    /// Turns an array of GET parameters into a single String
    pub (crate) fn parameters_to_string(parameters: Box<[(String, String)]>) -> String {
        if parameters.len() == 0 {
            return String::new();
        }

        let mut res = String::with_capacity(parameters.len() * 32 + 100);

        for (i, param) in parameters.into_iter().enumerate() {
            if i != 0 {
                res.push('&');
            }

            res.push_str(&param.0);
            res.push('=');

            let value = query_string_to_http(&param.1);
            res.push_str(&value);
        }

        res
    }
}

/// The way to transfer data over HTTP
pub (crate) enum TransferEncoding {
    ContentLength,
    Chunked,
}
