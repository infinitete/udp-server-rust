use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;

pub fn run(ip: [u8; 4], port: u16) {
    let addr = Ipv4Addr::from(ip);
    let socket_addr = SocketAddrV4::new(addr, port);

    let socket = UdpSocket::bind(socket_addr).expect("Error");
    let mut last_buff = [0;32];

    loop {
        let mut buf = [0;32];
        let (len, src) = socket.peek_from(&mut buf).expect("Receve failure");
        let (_, _) = socket.recv_from(&mut last_buff).expect("Receve failure");

        let is_fix = &buf[0..4];

        if (is_fix.eq(&[9;4])) {
            last_buff.reverse();
            socket.send_to(&last_buff, src).expect("Send failure");
            continue;
        }

        println!("Receve from {}: {} - {:?}", &src.ip(), len, &buf);
        buf.reverse();
        socket.send_to(&buf, src).expect("Send failure");
    }
}
