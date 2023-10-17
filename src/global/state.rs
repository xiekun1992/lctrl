use std::{ffi::c_int, sync::Mutex};

use super::device::{DeviceInfo, RemoteDevice};
use lazy_static::lazy_static;

#[link(name = "libcapture")]
extern "C" {
    fn get_screen_size(size: *const c_int);
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

pub struct State {
    pub remotes: Mutex<Vec<RemoteDevice>>,
    pub cur_device: DeviceInfo,
    pub remote_peer: Option<RemoteDevice>,
    pub screen_size: [i32; 2],
}

impl State {
    // pub fn get_instance() -> State {
    //     STATE.lock().unwrap().try_into()
    // }

    fn new() -> State {
        let mut screen_size = [0, 0];
        unsafe {
            get_screen_size(screen_size.as_mut_ptr());
        }
        State {
            remotes: Mutex::new(Vec::new()),
            cur_device: DeviceInfo::new(),
            remote_peer: None,
            screen_size,
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
