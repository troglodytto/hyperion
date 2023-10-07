use crate::error;
use crate::http::error::Error;
use crate::http::header::HttpHeader;
use crate::http::method::Method;
use serde::Serialize;
use std::fmt::Display;
use std::io::Read;
use std::net::TcpStream;

/// Representation of a HTTP Request
#[derive(Debug, Serialize, Clone)]
pub struct Request {
    /// The Method of the HTTP request
    pub method: Method,
    /// The Path of the HTTP request
    pub path: String,

    /// The HTTP Version used in the HTTP request
    pub http_version: String,

    /// A list of headers in the HTTP request
    pub headers: Vec<HttpHeader>,

    /// The body of the HTTP request
    pub body: Option<Vec<u8>>,
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = format!("{:?}", self.method).to_uppercase();

        write!(f, "{} {} {}\r\n", method, self.path, self.http_version)?;

        for header in &self.headers {
            write!(f, "{header}\r\n")?;
        }

        write!(f, "\r\n")?;

        if let Some(body) = &self.body {
            let serialized_body = String::from_utf8(body.clone()).map_err(|_| std::fmt::Error)?;

            write!(f, "{serialized_body}")?;
        }

        Ok(())
    }
}

/// Implementation of parsing an HTTP Response from a stream of bytes
pub trait HttpStream {
    /// Function that is used to parse a HTTP Request
    /// # Errors
    /// - If the stream contains invalid data
    /// - If something goes wrong during the parsing
    fn parse(&mut self) -> anyhow::Result<Request, Error>;
}

impl HttpStream for Vec<u8> {
    fn parse(&mut self) -> anyhow::Result<Request, Error> {
        let req_string = String::from_utf8(self.clone())?;

        let mut req_string = req_string.split("\r\n").filter(|line| !line.is_empty());

        let req_line = req_string
            .next()
            .ok_or(error!(BadRequest, "Invalid request line"))?;

        let mut req_line = req_line.split(' ');

        let method = req_line
            .next()
            .ok_or(error!(BadRequest, "Missing Method"))?;

        let path = req_line.next().ok_or(error!(BadRequest, "Invalid Path"))?;

        let http_version = req_line
            .next()
            .ok_or(error!(BadRequest, "Invalid HTTP Version"))?;

        // Leveraging serde to deserialize methods instead of manually converting from string to [HttpMethod]
        let method: Method = serde_json::from_str(&format!("\"{method}\""))
            .map_err(|_| error!(BadRequest, format!("Unknown HTTP Method: {method}")))?;

        let mut headers = vec![];

        for line in req_string {
            let Some((header_name, value)) = line.split_once(": ") else {
                return Err(error!(BadRequest, format!("Invalid Header: {line}")));
            };

            match HttpHeader::new(header_name, value)
                .map_err(|error| error!(BadRequest, error.to_string()))
            {
                Ok(h) => headers.push(h),
                Err(er) => println!("{er:?}"),
            };
        }

        Ok(Request {
            method,
            path: path.to_string(),
            http_version: http_version.to_string(),
            headers,
            body: None,
        })
    }
}

impl HttpStream for TcpStream {
    fn parse(&mut self) -> anyhow::Result<Request, Error> {
        let mut buffer = vec![0; 2048];
        let mut request_buffer = vec![];
        let mut body_buffer = vec![];

        'parse_loop: loop {
            let read_byte_count = self.read(&mut buffer)?;

            for (idx, window) in buffer.windows(4).enumerate() {
                if window == [13, 10, 13, 10] || read_byte_count < 4 {
                    request_buffer.extend(&buffer[..idx]);

                    // Add any remaining overflow to body bytes buffer
                    body_buffer.extend(&buffer[idx + 4..]);

                    break 'parse_loop;
                }
            }

            request_buffer.extend(&buffer[..read_byte_count]);
        }

        let mut request = request_buffer.parse()?;

        if let Some(content_length) = request
            .headers
            .iter()
            .find(|header| matches!(header, HttpHeader::ContentLength(_)))
            .and_then(|header| header.value().parse::<u16>().ok())
        {
            // This condition is to check the following
            // Do we need to extend? or is the overflow of bytes above it enough to store all the remaining body bytes
            // i.e Are there any remaining bytes to read other than the overflow above
            if content_length as usize > body_buffer.len() {
                buffer.clear();
                buffer.resize(content_length as usize, 0);

                let read_byte_count = self.read(&mut buffer)?;

                body_buffer.extend(&buffer[..read_byte_count]);
            }

            request.body = Some(body_buffer);
        }

        Ok(request)
    }
}
