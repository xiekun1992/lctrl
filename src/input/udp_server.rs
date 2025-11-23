use std::net::UdpSocket;
use std::{mem, slice, str};
use tracing::info;

pub struct UDPServer {
    socket: UdpSocket,
    _ip: String,
    _port: u16,
}

impl UDPServer {
    pub fn new(ip: String, port: u16) -> UDPServer {
        if let Ok(socket) = UdpSocket::bind(format!("{ip}:{port}")) {
            info!("UDP server bind to {}:{}", ip, port);
            UDPServer {
                socket,
                _ip: ip,
                _port: port,
            }
        } else {
            panic!("UDP server bind to {}:{} failed", ip, port);
        }
    }
    pub fn recv(&self, cb: fn(&[u32])) {
        // let dev = DeviceInfo::new();
        let mut buf = [0; 512];
        loop {
            if let Ok((recv_size, _rinfo)) = self.socket.recv_from(&mut buf) {
                let bytes = unsafe {
                    slice::from_raw_parts(
                        buf.as_ptr() as *const u32,
                        recv_size / mem::size_of::<u32>(),
                    )
                };
                {
                    cb(bytes);
                }
            }
        }
    }
    pub fn send(&self, bytes: &[u8], addr: &str) {
        self.socket.send_to(bytes, addr).expect("send input failed");
    }
}
