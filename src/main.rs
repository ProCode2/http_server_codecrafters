mod http;
use std::{collections::HashMap, env, fs, path, sync::Arc};

use http::Server;

use crate::http::{HTTPVersion, Request, RequestBody, Response, StatusCode};

fn main() -> Result<(), std::io::Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.

    // Uncomment this block to pass the first stage
    let mut server = Server::new("127.0.0.1:4221")?;
    server.add_route(http::Method::GET, "/", |req: &Request| {
        Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::Ok)
    });

    server.add_route(http::Method::GET, "/echo/{cont}", |req: &Request| {
        let mut headers: HashMap<String, String> = HashMap::new();
        let params = req.get_params();
        let d = String::new();
        let content = params.get("cont").unwrap_or(&d);
        headers.insert(String::from("Content-Type"), String::from("text/plain"));
        headers.insert(String::from("Content-Length"), content.len().to_string());
        let mut res = Response::new(HTTPVersion::HTTP1_1, headers, StatusCode::Ok);
        res.set_body(RequestBody::String(content.as_bytes().to_vec()));

        res
    });

    server.add_route(http::Method::GET, "/user-agent", |req: &Request| {
        let mut headers: HashMap<String, String> = HashMap::new();
        let content = req.get_headers().get("user-agent").unwrap();
        headers.insert(String::from("Content-Type"), String::from("text/plain"));
        headers.insert(String::from("Content-Length"), content.len().to_string());
        let mut res = Response::new(HTTPVersion::HTTP1_1, headers, StatusCode::Ok);
        res.set_body(RequestBody::String(content.as_bytes().to_vec()));

        res
    });

    server.add_route(http::Method::GET, "/files/{file_name}", |req: &Request| {
        let dir = env::args().last().unwrap_or("/tmp/".to_string());

        let params = req.get_params();
        let d = String::new();
        let file_name = params.get("file_name").unwrap_or(&d);
        let p = path::PathBuf::from(format!("{}{}", &dir, &file_name));

        let mut headers: HashMap<String, String> = HashMap::new();
        if p.exists() && p.is_file() {
            let content = fs::read_to_string(p).unwrap_or("".to_string());
            headers.insert(
                String::from("Content-Type"),
                String::from("application/octet-stream"),
            );
            headers.insert(String::from("Content-Length"), content.len().to_string());
            let mut res = Response::new(HTTPVersion::HTTP1_1, headers, StatusCode::Ok);
            res.set_body(RequestBody::String(content.as_bytes().to_vec()));
            res
        } else {
            Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::NotFound)
        }
    });

    server.add_route(http::Method::POST, "/files/{file_name}", |req: &Request| {
        let dir = env::args().last().unwrap_or("/tmp/".to_string());

        let params = req.get_params();
        let d = String::new();
        let file_name = params.get("file_name").unwrap_or(&d);
        let p = path::PathBuf::from(format!("{}{}", &dir, &file_name));
        let body = req.get_body();
        if let Some(body) = body {
            match body {
                RequestBody::String(bytes) => match fs::write(p, bytes) {
                    Ok(_) => {
                        Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::Created)
                    }
                    Err(_) => {
                        Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::NotFound)
                    }
                },
            }
        } else {
            Response::new(HTTPVersion::HTTP1_1, HashMap::new(), StatusCode::NotFound)
        }
    });

    server.run();

    Ok(())
}
