use std::net::UdpSocket;
use std::{slice, str};

pub struct UDPServer {
    socket: UdpSocket,
    ip: String,
    port: u16,
}

impl UDPServer {
    pub fn new(ip: String, port: u16) -> UDPServer {
        let socket = UdpSocket::bind(format!("{ip}:{port}")).unwrap();
        // socket.set_nonblocking(true).unwrap();
        // socket.set_broadcast(true).unwrap();
        UDPServer { socket, ip, port }
    }
    pub fn recv(&self, cb: fn(&[u8])) {
        // let dev = DeviceInfo::new();
        let mut buf = [0; 512];
        loop {
            let (recv_size, rinfo) = self.socket.recv_from(&mut buf).unwrap();
            let bytes = unsafe { slice::from_raw_parts(buf.as_ptr(), recv_size) };
            {
                cb(bytes);
            }
        }
    }
    pub fn send(&self, bytes: &[u8], addr: &str) {
        self.socket.send_to(bytes, addr).expect("send input failed");
    }
}
