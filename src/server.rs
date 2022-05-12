use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::{Write, Read};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        println!("Listening on {}", self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some("<h1>Olá</h1>".to_string()))
                                },
                                Err(e) => {
                                    println!("Failed to match the request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                },
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to stablish connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to stablish connection: {}", e)
                }
            }
        }
    }
}
