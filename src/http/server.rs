use std::{
    any::Any,
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::AsFd,
};

use itertools::Itertools;

use crate::http::request::Request;

use super::{
    error::HTTPError,
    request::{HTTPVersion, Method, RequestBody, RequestTarget},
    response::{Response, StatusCode},
};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    current_stream: Option<TcpStream>,
}

impl Server {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        println!("Started listening from the server");

        Ok(Self {
            listener,
            current_stream: None,
        })
    }

    fn set_stream(&mut self, stream: TcpStream) {
        self.current_stream = Some(stream);
    }

    fn clear_stream(&mut self) {
        self.current_stream = None;
    }

    fn read_request(&mut self) -> Result<Request, HTTPError> {
        let buf_reader = BufReader::new(self.current_stream.as_ref().unwrap());
        let mut bytes: Vec<u8> = Vec::new();
        let mut byte_iter = buf_reader.bytes();
        while let Some(byte) = byte_iter.next() {
            // read only until header and parse the body later according to contet length and type
            if bytes.len() >= 4 && &bytes[(bytes.len() - 3)..] == b"\r\n\r" {
                break;
            }
            bytes.push(byte.unwrap());
        }
        Ok(Request::from(bytes))
    }

    fn return_response(&mut self, res: &[u8]) -> Result<(), std::io::Error> {
        self.current_stream.as_ref().unwrap().write_all(&res[..])?;
        Ok(())
    }

    fn process_request(&mut self, req: Request) -> Response {
        if req.get_target() == String::from("/") {
            Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::Ok)
        } else if req.get_target().starts_with("/echo") {
            let mut headers: HashMap<String, String> = HashMap::new();
            let content = req.get_target().replace("/echo/", "");
            headers.insert(String::from("Content-Type"), String::from("text/plain"));
            headers.insert(String::from("Content-Length"), content.len().to_string());
            let mut res = Response::new(HTTPVersion::HTTP1_1, headers, StatusCode::Ok);
            res.set_body(RequestBody::String(content.as_bytes().to_vec()));

            res
        } else if req.get_target() == String::from("/user-agent") {
            let mut headers: HashMap<String, String> = HashMap::new();
            let content = req.get_headers().get("user-agent").unwrap();
            headers.insert(String::from("Content-Type"), String::from("text/plain"));
            headers.insert(String::from("Content-Length"), content.len().to_string());
            let mut res = Response::new(HTTPVersion::HTTP1_1, headers, StatusCode::Ok);
            res.set_body(RequestBody::String(content.as_bytes().to_vec()));

            res
        } else {
            Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::NotFound)
        }
    }

    pub fn run(&mut self) {
        for stream in self.listener.try_clone().unwrap().incoming() {
            match stream {
                Ok(stream) => {
                    self.set_stream(stream);
                    println!(
                        "Connected to server: Client: {:?}",
                        self.current_stream.as_ref().unwrap().type_id()
                    );
                    let req = self.read_request();

                    match req {
                        Ok(req) => {
                            let resp = self.process_request(req);
                            println!("{}", resp);
                            self.return_response(resp.to_string().as_bytes());
                        }
                        Err(_) => {
                            eprintln!("Error in parsing request");
                        }
                    };

                    self.clear_stream();
                }

                Err(err) => {
                    eprintln!("Something went wrong: {}", err);
                }
            }
        }
    }
}
