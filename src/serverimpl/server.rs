use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{fs, thread};
use std::time::Duration;
use crate::serverimpl::ThreadPool;

pub struct Server {
    address: String,
    thread_pool: ThreadPool,
}

impl Server {
    pub fn new(address: String, thread_pool: ThreadPool) -> Self {
        Server {
            address,
            thread_pool,
        }
    }

    pub fn run(self) {
        let tcp_listener = TcpListener::bind(&self.address).unwrap();
        loop {
            match tcp_listener.accept() {
                Ok((stream, _)) => {
                    thread::spawn(|| {
                        self.handle_connection(stream);
                    });
                }
                Err(e) => println!("TCP connection failed {}", e)
            }
        }
    }

    fn handle_connection(self, mut stream: TcpStream) {
        println!("TCP connection established! {}",
                 self.address);
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Received a request: {}",
                         String::from_utf8_lossy( &buffer[..]));
                let response = self.deserialize_and_response(&buffer);
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => println!("Failed to read from connection: {}", e)
        }
    }

    fn deserialize_and_response(self, &buffer: &[u8; 1024]) -> String {
        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (status_line, file_name) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "some_file.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "some_file.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let content = fs::read_to_string(file_name).unwrap();
        format! (
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            content.len(),
            content
        )
    }
}

