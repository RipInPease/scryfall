use std::io::{Read, Write, Result as IOResult};
use std::collections::HashMap;

use super::{Error, Response, query_string_to_http};

/// A connection used to send REST API requests and read responses from 
/// using HTTP 1.1 
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
    /// Sends a REST API request and reads the response
    pub fn rest_request(&mut self, request: RestRequest) -> Result<Response, Error> {
        let mut data = String::with_capacity(8192);

        let header_string = Self::headers_to_string(&self.headers);

        self.write_all(data.as_bytes())?;
        let response = Response::read_from_stream(self)?;
        
        Ok(response)
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
}

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
        data: String
    },

    /// Update or create a resource
    PUT {
        path: String,
        data: String,
    },

    /// Partially update a resource
    PATCH {
        path: String,
        data: String,
    },

    DELETE {
        path: String
    }
}

impl RestRequest {
    /// Returns a string of how the REST API request would look.
    /// If there is any data to be sent in the data portion of the HTTP request,
    /// it would also return that
    pub (crate) fn string_and_data(self) -> (String, Option<String>) {
        let mut res = String::with_capacity(512);
        let mut res_data = None;

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
                res_data = Some(data)
            },
            Self::POST { path, data } => {
                res.push_str("POST ");
                res.push_str(&path);
                res_data = Some(data)
            },
            Self::PUT { path, data } => {
                res.push_str("PUT ");
                res.push_str(&path);
                res_data = Some(data)
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