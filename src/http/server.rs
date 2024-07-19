use std::{
    any::Any,
    collections::HashMap,
    env, fs,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path,
    sync::{Arc, Mutex},
    thread,
};

use crate::http::request::Request;

use super::{
    error::HTTPError,
    request::{HTTPVersion, RequestBody},
    response::{Response, StatusCode},
    router::Router,
    Encoding, Method,
};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    pub router: Arc<Mutex<Router>>,
}

impl Server {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        println!("Started listening from the server");
        let router = Arc::new(Mutex::new(Router::new()));
        Ok(Self { listener, router })
    }

    pub fn add_route<F>(&self, method: Method, p: &str, f: F)
    where
        F: Fn(&Request) -> Response + 'static + Send + Sync,
    {
        self.router.lock().unwrap().add_route(method, p, f);
    }

    fn read_request(stream: TcpStream) -> Result<Request, HTTPError> {
        let buf_reader = BufReader::new(stream);
        let mut bytes: Vec<u8> = Vec::new();
        let mut byte_iter = buf_reader.bytes();
        while let Some(byte) = byte_iter.next() {
            // read only until header and parse the body later according to contet length and type
            if bytes.len() >= 4 && &bytes[(bytes.len() - 3)..] == b"\r\n\r" {
                break;
            }
            bytes.push(byte.unwrap());
        }
        let mut req = Request::from(bytes);

        // read body based on Content-Length property
        let headers = req.get_headers();
        let length = match headers
            .get("content-length")
            .unwrap_or(&String::new())
            .parse::<usize>()
        {
            Ok(l) => l,
            Err(_) => 0,
        };

        let body_bytes = byte_iter
            .take(length)
            .map(|x| x.unwrap())
            .collect::<Vec<u8>>();
        req.set_body(RequestBody::String(Vec::from(body_bytes)));
        Ok(req)
    }

    fn return_response(mut stream: TcpStream, res: &[u8]) -> Result<(), std::io::Error> {
        stream.write_all(&res[..])?;
        Ok(())
    }

    fn process_request(mut req: Request, router: &Router) -> Response {
        let (params, handler) =
            router.get_handler_and_params(req.get_method(), req.get_target().as_str());
        req.set_params(params);
        match handler {
            Some(h) => {
                let mut res = h(&req);
                let h = req.get_headers();
                let mut headers = res.get_headers();
                let d = String::new();
                if h.get("accept-encoding").is_some() {
                    let encoding_str = h.get("accept-encoding").unwrap_or(&d);
                    let encoding = Encoding::get_endoing_scheme(encoding_str);
                    if let Some(enc) = encoding {
                        headers.insert("Content-Encoding".to_string(), enc.to_string());
                    }
                }

                headers.insert("Content-Type".to_string(), "text/plain".to_string());
                res.set_headers(headers);
                res
            }
            None => Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::NotFound),
        }
    }

    fn handle_connection(stream: TcpStream, router: Arc<Mutex<Router>>) {
        println!("Connected to server: Client: {:?}", stream.type_id());
        let req = Server::read_request(stream.try_clone().unwrap());

        match req {
            Ok(req) => {
                let resp = Server::process_request(req, &Arc::clone(&router).lock().unwrap());
                Server::return_response(stream, resp.to_string().as_bytes());
            }
            Err(_) => {
                eprintln!("Error in parsing request");
            }
        };
    }

    pub fn run(&mut self) {
        for stream in self.listener.try_clone().unwrap().incoming() {
            let router = Arc::clone(&self.router);
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        Server::handle_connection(stream.try_clone().unwrap(), router)
                    });
                    ()
                }

                Err(err) => {
                    eprintln!("Something went wrong: {}", err);
                }
            }
        }
    }
}
