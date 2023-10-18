use std::str;
use std::time::Duration;
use std::{net::UdpSocket, thread};

use crate::global::device::RemoteDevice;
use crate::global::state::STATE;

pub struct UDPServer {
    socket: UdpSocket,
    _ip: String,
    port: u16,
}

impl UDPServer {
    pub fn new(ip: String, port: u16) -> UDPServer {
        let socket = UdpSocket::bind(format!("{ip}:{port}")).unwrap();
        // socket.set_nonblocking(true).unwrap();
        socket.set_broadcast(true).unwrap();
        UDPServer {
            socket,
            _ip: ip,
            port,
        }
    }
    pub fn recv(&self) {
        let mut buf = [0; 64 * 1024];
        loop {
            let (data, rinfo) = self.socket.recv_from(&mut buf).unwrap();
            let remote = RemoteDevice::from_json(str::from_utf8(&buf[..data]).unwrap().to_string());

            let state = STATE.lock().unwrap();
            let dev = &state.cur_device;
            if dev
                .ifs
                .iter()
                .all(|interface| interface.addr.to_string() != rinfo.ip().to_string())
            {
                state.add_remote(remote);
            }
        }
    }
    pub fn send(&self) {
        let mut remote_infos = Vec::new();
        {
            let state = STATE.lock().unwrap();
            let dev = &state.cur_device;
            for interface in &dev.ifs {
                let addr = format!("{}:{}", interface.broadcast_addr, self.port);
                let remote = RemoteDevice {
                    hostname: dev.hostname.clone(),
                    ip: interface.addr.to_string(),
                    screen_size: state.screen_size.clone(),
                }
                .to_json();
                remote_infos.push((remote, addr));
            }
        }
        loop {
            for (remote, addr) in &remote_infos {
                self.socket
                    .send_to(remote.as_bytes(), addr)
                    .expect("send failed");
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}
