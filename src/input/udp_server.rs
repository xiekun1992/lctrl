use std::sync::Arc;
use std::time::Duration;
use std::{net::UdpSocket, thread};
use std::{slice, str};

use crate::global::device::{DeviceInfo, RemoteDevice};
use crate::global::state::{State, STATE};
use chrono::prelude::*;

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
            let bytes = unsafe {
                slice::from_raw_parts(buf.as_ptr(), recv_size)
            };
            // let remote = DeviceInfo::from_json(str::from_utf8(&buf[..data]).unwrap().to_string());

            // let state = STATE.lock().unwrap();
            // let dev = &state.cur_device;
            // if dev
            //     .ifs
            //     .iter()
            //     .all(|interface| interface.addr.to_string() != rinfo.ip().to_string())
            {
                println!("{} - {:?}", Local::now(), bytes);
                cb(bytes);
            }
        }
    }
    pub fn send(&self, bytes: &[u8], addr: &str) {
        self.socket.send_to(bytes, addr).expect("send input failed");
    }
}
