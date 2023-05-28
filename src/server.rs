use std::io::{Read};
use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
// use std::convert::TryInto;

pub struct Server {
    socket_addr: String,
}

impl Server {
    pub fn new(socket_addr: String) -> Self {
        Self {
            socket_addr
        }
    }

    pub fn run(self) {
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
                                Ok(r) => {
                                    dbg!(r);
                                    Response::new(StatusCode::Ok, Some("Hello".to_string()))
                                }
                                Err(e) => {
                                    print!("Error reading stream: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
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
