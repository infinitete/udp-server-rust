use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;

pub fn run(ip: [u8; 4], port: u16) {
    let addr = Ipv4Addr::from(ip);
    let socket_addr = SocketAddrV4::new(addr, port);

    let socket = UdpSocket::bind(socket_addr).expect("Error");

    loop {
        let mut buf = [0;32];
        let (_len, src) = socket.recv_from(&mut buf).expect("Receve failure");
        buf.reverse();
        socket.send_to(&buf, src).expect("Send failure");
    }
}
