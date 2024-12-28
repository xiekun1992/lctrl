pub mod udp_server;

use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tracing::{debug, error, info};
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

        match STATE.lock() {
            Ok(state) => {
                let peer_in_manual_remotes =
                    if let Ok(manual_remotes) = state.manual_remotes.try_lock() {
                        match state.remote_peer {
                            Some(ref rdev) => {
                                if manual_remotes
                                    .iter()
                                    .find(|item| item.ip.eq(&rdev.ip))
                                    .is_some()
                                {
                                    Some(())
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };

                let peer_in_remotes = if let Ok(mut remotes) = state.remotes.try_lock() {
                    remotes.retain(|item| {
                        // println!("{}, {}", timestamp, item.alive_timestamp);
                        timestamp.wrapping_sub(item.alive_timestamp) < 3u64
                    });
                    match state.remote_peer {
                        Some(ref rdev) => {
                            if remotes.iter().find(|item| item.ip.eq(&rdev.ip)).is_some() {
                                Some(())
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                } else {
                    None
                };

                if peer_in_manual_remotes.is_none() && peer_in_remotes.is_none() {
                    crate::input::listener::release();
                } else {
                    crate::input::listener::keepalive();
                }
            }
            Err(err) => {
                error!("{:?}", err);
            }
        }
    });

    info!("discover module init");
}
