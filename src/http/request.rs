use super::error::HTTPError;
use std::fmt;
use std::{collections::HashMap, result::Result, str::FromStr};
/**

// Request line
GET                          // HTTP method
/index.html                  // Request target
HTTP/1.1                     // HTTP version
\r\n                         // CRLF that marks the end of the request line

// Headers
Host: localhost:4221\r\n     // Header that specifies the server's host and port
User-Agent: curl/7.64.1\r\n  // Header that describes the client's user agent
Accept: \r\n              // Header that specifies which media types the client can accept
\r\n                         // CRLF that marks the end of the headers

// Request body (empty)**/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    UNKNOWN,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            _ => Method::UNKNOWN,
        }
    }
}

#[derive(Debug)]
pub enum HTTPVersion {
    HTTP1_1,
    UNSUPPORTED,
}

impl fmt::Display for HTTPVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HTTPVersion::HTTP1_1 => write!(f, "HTTP/1.1"),
            HTTPVersion::UNSUPPORTED => write!(f, "UNSUPPORTED"),
        }
    }
}

impl From<&str> for HTTPVersion {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => HTTPVersion::HTTP1_1,
            _ => HTTPVersion::UNSUPPORTED,
        }
    }
}

#[derive(Debug)]
pub enum RequestTarget {
    OriginForm(String),
}

impl From<&str> for RequestTarget {
    fn from(s: &str) -> Self {
        match s.len() {
            0 => RequestTarget::OriginForm("/".to_string()),
            _ => RequestTarget::OriginForm(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum RequestBody {
    String(Vec<u8>),
}

impl fmt::Display for RequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestBody::String(bytes) => {
                write!(f, "{}", String::from_utf8(bytes.clone()).unwrap())
            }
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    target: RequestTarget,
    version: HTTPVersion,
    headers: HashMap<String, String>,
    body: Option<RequestBody>,
    params: HashMap<String, String>,
}

impl Request {
    pub fn get_target(&self) -> String {
        match &self.target {
            RequestTarget::OriginForm(s) => s.to_owned(),
        }
    }

    pub fn get_method(&self) -> Method {
        self.method
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }

    pub fn get_params(&self) -> HashMap<String, String> {
        self.params.clone()
    }

    fn separate_body_from_request(bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
        // Find the position of the double CRLF that separates the headers from the body
        let mut headers_end = None;
        if bytes.len() >= 3 {
            for i in 0..bytes.len() - 3 {
                if &bytes[i..i + 4] == b"\r\n\r\n" {
                    headers_end = Some(i + 4);
                    break;
                }
            }
        }

        // Split the bytes into headers and body based on the found position
        if let Some(headers_end) = headers_end {
            let headers = bytes[..headers_end].to_vec();
            let body = bytes[headers_end..].to_vec();
            (headers, body)
        } else {
            // If no headers/body separator is found, return the whole data as headers and an empty body
            (bytes.to_vec(), Vec::new())
        }
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

impl From<Vec<u8>> for Request {
    fn from(bytes: Vec<u8>) -> Self {
        // seprate body because it may contain not utf8 elements
        // TODO: parse body based on content-type
        let (up_to_header, body) = Request::separate_body_from_request(&bytes[..]);

        let mut req = String::from_utf8(up_to_header)
            .unwrap()
            .parse::<Request>()
            .expect("Can not parse header");
        req.body = Some(RequestBody::String(Vec::from(body)));
        req
    }
}

impl FromStr for Request {
    type Err = HTTPError;
    fn from_str(s: &str) -> Result<Self, HTTPError> {
        if let Some((first_line, rest_of_req)) = s.split_once("\r\n") {
            // build http request

            // parse http method
            let method_target_version: Vec<&str> = first_line.split(" ").collect();
            let method = match method_target_version.get(0) {
                Some(method) => Method::from(*method),
                None => {
                    return Err(HTTPError::Custom);
                }
            };

            // parse http target
            let target = match method_target_version.get(1) {
                Some(tar) => RequestTarget::from(*tar),
                None => {
                    return Err(HTTPError::Custom);
                }
            };

            // parse http version
            let version = match method_target_version.get(2) {
                Some(ver) => HTTPVersion::from(*ver),
                None => {
                    return Err(HTTPError::Custom);
                }
            };

            // parse headers
            let mut headers = HashMap::new();
            rest_of_req
                .split("\r\n")
                .take_while(|header_line| header_line != &"")
                .for_each(|header_line| {
                    let split: Vec<&str> = header_line.split(":").collect();
                    if split.len() < 2 {
                        return;
                    }
                    let key = split.first().unwrap().trim().to_lowercase().to_string();
                    let value = split.get(1).unwrap().trim().to_string();
                    headers.insert(key, value);
                });

            Ok(Self {
                method,
                target,
                version,
                headers,
                body: None,
                params: HashMap::new(),
            })
        } else {
            Err(HTTPError::Custom)
        }
    }
}
