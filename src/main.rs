use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};

fn main() {
    let adresses = [
        SocketAddr::from(([127, 0, 0, 1], 7878)),
        // SocketAddr::from(([127, 0, 0, 1], 667)),
    ];
    let listener = TcpListener::bind(&adresses[..]).unwrap();
    assert_eq!(listener.local_addr().unwrap(),
               SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1),
                                                7878)));
    for stream in listener.incoming() {
        let _stream = stream;
        println!("TCP connection established! {}", listener.local_addr().unwrap());
    }
}
