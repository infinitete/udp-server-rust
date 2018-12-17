use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;

pub fn run(ip: [u8; 4], port: u16) {
    let addr = Ipv4Addr::from(ip);
    let socket_addr = SocketAddrV4::new(addr, port);

    let socket = UdpSocket::bind(socket_addr).expect("Error");
    let mut last_buff = [0;32];

    loop {
        let mut buff = [0;32];
        let (len, src) = socket.peek_from(&mut buff).expect("Receve failure");
        last_buff = buff.clone();
        let is_fix = &buff[0..4];

        if is_fix.eq(&[9;4]) {
            let mut last = last_buff.clone();
            last.reverse();
            println!("Fix: ");
            socket.send_to(&last_buff, src).expect("Send failure");
            continue;
        }

        println!("Receve from {}: {} - {:?}", &src.ip(), len, &buff);
        buff.reverse();
        socket.send_to(&buff, src).expect("Send failure");
    }
}
