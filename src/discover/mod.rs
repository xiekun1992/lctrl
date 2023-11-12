pub mod udp_server;

use std::{sync::Arc, thread};
use udp_server::UDPServer;

pub fn init() {
    let udp_discover = UDPServer::new("0.0.0.0".to_string(), 11232);
    let server = Arc::new(udp_discover);

    let recv_server = Arc::clone(&server);
    thread::spawn(move || {
        recv_server.recv();
    });

    let send_server = Arc::clone(&server);
    thread::spawn(move || {
        send_server.send();
    });
}
