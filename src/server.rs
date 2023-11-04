use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::net::{TcpListener, TcpStream};
use std::{io::Read, net::UdpSocket};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    socket_addr: String,
}

impl Server {
    pub fn new(socket_addr: String) -> Self {
        Self { socket_addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("{}", self.socket_addr);

        let listener = TcpListener::bind(&self.socket_addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(..) => {
                            println!("received our request: {}", String::from_utf8_lossy(&buf));
                            let resp = match Request::try_from(&buf[..]) {
                                Ok(r) => handler.handle_request(&r),
                                Err(e) => {
                                    print!("Error reading stream: {}", e);
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = resp.send(&mut stream) {
                                print!("Error sending to stream: {}", e)
                            }
                        }
                        Err(e) => {
                            println!("failed to read from connection: {}", e)
                        }
                    }
                }
                Err(err) => {
                    println!("failed to establish connection: {}", err);
                    continue;
                }
            }
        }
    }
}
