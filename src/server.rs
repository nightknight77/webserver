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
            let tup = tcp_listener.accept();
            match tup {
                Ok((mut tcp_stream, _)) => {
                    println!("TCP connection established! {}",
                             self.address);
                    let mut buffer = [0;1024];
                    let request = tcp_stream.read(&mut buffer);
                    println!("{}", request.unwrap().to_string());
                },
                Err(e) => println!("TCP connection failed {}", e)
            }
        }
    }
}

