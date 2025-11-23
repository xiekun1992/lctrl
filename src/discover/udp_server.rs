use std::str;
use std::time::Duration;
use std::{net::UdpSocket, thread};

use crate::global::device::RemoteDevice;
use crate::global::state::STATE;
use tracing::info;

pub struct UDPServer {
    socket: UdpSocket,
    _ip: String,
    port: u16,
}

impl UDPServer {
    pub fn new(ip: String, port: u16) -> UDPServer {
        if let Ok(socket) = UdpSocket::bind(format!("{ip}:{port}")) {
            if let Ok(_) = socket.set_broadcast(true) {
                info!("UDP server bind to {}:{}", ip, port);
            } else {
                panic!("set broadcast failed");
            }
            UDPServer {
                socket,
                _ip: ip,
                port,
            }
        } else {
            panic!("UDP server bind to {}:{} failed", ip, port);
        }
    }
    pub fn recv(&self) {
        let mut buf = [0; 64 * 1024];
        loop {
            if let Ok((data, rinfo)) = self.socket.recv_from(&mut buf) {
                if let Ok(s) = str::from_utf8(&buf[..data]) {
                    let remote = RemoteDevice::from_json(s.to_string());
                    // println!("{:?}", remote);
                    match STATE.lock() {
                        Ok(mut state) => {
                            let dev = &state.cur_device;
                            // println!("{:?}, {:?}", dev, remote);
                            if dev.ifs.iter().all(|interface| {
                                interface.addr.to_string() != rinfo.ip().to_string()
                            }) {
                                state.add_remote(remote);
                            }
                        }
                        Err(_e) => {}
                    }
                }
            }
        }
    }
    pub fn send(&self) {
        loop {
            let mut remote_infos = Vec::new();

            if let Ok(state) = STATE.lock() {
                if state.setting.auto_discover {
                    let dev = &state.cur_device;
                    for interface in &dev.ifs {
                        let addr = format!("{}:{}", interface.broadcast_addr, self.port);
                        let remote = RemoteDevice {
                            hostname: dev.hostname.clone(),
                            ip: interface.addr.to_string(),
                            mac_addr: interface.mac_addr.clone(),
                            screen_size: state.screen_size.clone(),
                            netmask: interface.netmask.to_string(),
                            screens: state.screens.clone(),
                            alive_timestamp: 0,
                        }
                        .to_json();
                        remote_infos.push((remote, addr));
                    }
                }
            }
            for (remote, addr) in &remote_infos {
                match self.socket.send_to(remote.as_bytes(), addr) {
                    Err(e) => {
                        println!("discover packet send failed: {}", e);
                    }
                    _ => {}
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    }
}
