pub mod udp_server;

use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use udp_server::UDPServer;

use crate::{
    global::state::STATE,
    input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE},
};

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
        {
            let state = STATE.try_lock().unwrap();
            {
                let mut remotes = state.remotes.try_lock().unwrap();
                remotes.retain(|item| {
                    // println!("{}, {}", timestamp, item.alive_timestamp);
                    (timestamp - item.alive_timestamp) < 5u64
                });
                // println!("{:?}", remotes);
                match state.remote_peer.clone() {
                    Some(rdev) => {
                        if remotes.iter().find(|item| item.ip.eq(&rdev.ip)).is_none() {
                            unsafe {
                                REMOTE_SCREEN_SIZE = [0, 0];
                                SELF_SCREEN_SIZE = [0, 0];
                                SIDE = ControlSide::NONE;
                            }
                        }
                    }
                    None => {}
                }
            }
        }
    });
}
