use std::io::Read;
use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

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
                            match Request::try_from(&buf[..]) {
                                Ok(..) => {

                                }
                                Err(err) => {
                                    println!("error converting from [u8] to Request: {}", err)
                                }
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
