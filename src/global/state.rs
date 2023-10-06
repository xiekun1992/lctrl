use std::sync::{Arc, Mutex};

use super::device::{DeviceInfo, RemoteDevice};
use lazy_static::lazy_static;

lazy_static! {
   pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

pub struct State {
    pub remotes: Mutex<Vec<RemoteDevice>>,
    pub cur_device: DeviceInfo,
    pub remote_peer: Option<RemoteDevice>,
}

impl State {
    // pub fn get_instance() -> State {
    //     STATE.lock().unwrap().try_into()
    // }

    fn new() -> State {
        State {
            remotes: Mutex::new(Vec::new()),
            cur_device: DeviceInfo::new(),
            remote_peer: None,
        }
    }

    pub fn set_remote_peer(&mut self, peer: Option<RemoteDevice>) {
        self.remote_peer = peer;
    }

    pub fn get_remote(&self) -> Vec<RemoteDevice> {
        self.remotes.lock().unwrap().clone()
    }

    pub fn add_remote(&self, remote: RemoteDevice) {
        let mut remotes = self.remotes.lock().unwrap();
        if !remotes.contains(&remote) {
            remotes.push(remote);
        }
    }

    pub fn find_remote_by_ip(&self, ip: &str) -> Option<RemoteDevice> {
        let remotes = self.remotes.lock().unwrap();
        for r in remotes.clone().into_iter() {
            if r.ip.as_str() == ip {
                return Some(r.clone());
            }
        }
        None
    }
}
