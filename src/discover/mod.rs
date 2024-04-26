pub mod udp_server;

use log::{debug, error, info};
use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use udp_server::UDPServer;

use crate::global::state::STATE;

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

    thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(1));
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => {
                println!("SystemTime before UNIX EPOCH!");
                0
            }
        };

        match STATE.try_lock() {
            Ok(state) => {
                let mut remotes = state.remotes.try_lock().unwrap();
                remotes.retain(|item| {
                    // println!("{}, {}", timestamp, item.alive_timestamp);
                    (timestamp - item.alive_timestamp) < 5u64
                });
                // debug!("{:?} {:?}", remotes, state.remote_peer);
                match state.remote_peer.clone() {
                    Some(rdev) => {
                        if remotes.iter().find(|item| item.ip.eq(&rdev.ip)).is_none() {
                            crate::input::listener::release();
                        } else {
                            crate::input::listener::keepalive();
                        }
                    }
                    None => {}
                }
            }
            Err(err) => {
                error!("{:?}", err);
            }
        }
    });

    info!("discover module init");
}
