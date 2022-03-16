use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server {
            address
        }
    }

    pub fn run(self) {
        let tcp_listener = TcpListener::bind(&self.address).unwrap();
        loop {
            match tcp_listener.accept() {
                Ok((mut stream, _)) => {
                    println!("TCP connection established! {}",
                             self.address);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => println!("Received a request: {}",
                                                String::from_utf8_lossy(&buffer[..])),
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                }
                Err(e) => println!("TCP connection failed {}", e)
            }
        }
    }
}

