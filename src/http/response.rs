use std::collections::HashMap;
use std::fmt;

use super::request::{HTTPVersion, RequestBody};

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    Created,
    NotFound,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "200 OK"),
            StatusCode::NotFound => write!(f, "404 Not Found"),
            StatusCode::Created => write!(f, "201 Created"),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    version: HTTPVersion,
    headers: HashMap<String, String>,
    status_code: StatusCode,
    body: Option<RequestBody>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the status line
        write!(f, "{} {}\r\n", self.version, self.status_code.to_string())?;

        // Write the headers
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }

        // Write a blank line to separate headers from the body
        write!(f, "\r\n")?;

        if self.body.is_some() {
            // write the body
            write!(f, "{}", self.body.as_ref().unwrap().to_string())?;
        }

        Ok(())
    }
}

impl Response {
    pub fn new(
        version: HTTPVersion,
        headers: HashMap<String, String>,
        status_code: StatusCode,
    ) -> Self {
        Self {
            version,
            headers,
            status_code,
            body: None,
        }
    }

    pub fn set_body(&mut self, body: RequestBody) {
        self.body = Some(body);
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    pub fn set_headers(&mut self, h: HashMap<String, String>) {
        self.headers = h;
    }
}
