use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
use std::fs;

pub struct WebHandler{
    public_path: String
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, filepath: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, filepath);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal was attempted {}", filepath);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/tester" => Response::new(StatusCode::Ok, Some("testing testing".to_string())),
                path => match self.read_file(path) {
                    Some(s) => Response::new(StatusCode::Ok, Some(s)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            Method::POST => Response::new(StatusCode::Ok, Some("postman".to_string())),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}