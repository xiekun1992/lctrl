use std::{ffi::c_int, sync::Mutex};

use crate::input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE};

use super::{
    db::DB_CONN,
    device::{DeviceInfo, RemoteDevice},
};
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
    pub side: ControlSide,
}

impl State {
    fn new() -> State {
        let mut screen_size = [0, 0];
        unsafe {
            get_screen_size(screen_size.as_mut_ptr());
        }
        State {
            remotes: Mutex::new(Vec::new()),
            cur_device: DeviceInfo::new(),
            screen_size,
            remote_peer: None,
            side: ControlSide::NONE,
        }
    }

    pub fn get_remote_peer(&mut self) -> Option<RemoteDevice> {
        let db = DB_CONN.lock().unwrap();
        let (remote_peer, side) = db.get_remote_peer();
        match remote_peer.clone() {
            Some(rdev) => {
                if self.find_remote_by_ip(&rdev.ip).is_some() {
                    unsafe {
                        REMOTE_SCREEN_SIZE = rdev.screen_size.clone();
                        SELF_SCREEN_SIZE = self.screen_size.clone();
                        SIDE = side;
                    }
                }
            }
            None => {}
        }
        self.remote_peer = remote_peer;
        self.remote_peer.clone()
    }

    pub fn set_remote_peer(&mut self, peer: Option<RemoteDevice>, side: &ControlSide) {
        match peer.clone() {
            Some(ref r) => {
                let db = DB_CONN.lock().unwrap();
                db.set_remote_peer(r, side);
            }
            None => {
                let db = DB_CONN.lock().unwrap();
                db.delete_remote_peer();
            }
        }
        self.remote_peer = peer;
    }

    pub fn get_remote(&self) -> Vec<RemoteDevice> {
        self.remotes.lock().unwrap().clone()
    }

    pub fn add_remote(&mut self, remote: RemoteDevice) {
        {
            let mut remotes = self.remotes.lock().unwrap();
            if !remotes.contains(&remote) {
                remotes.push(remote);
                // println!("{:?}, {:?}", remotes, self.remote_peer);
            }
        }
        self.get_remote_peer();
    }

    pub fn del_remote(&self, ip: String) {
        let mut remotes = self.remotes.lock().unwrap();
        remotes.retain(|item| item.ip != ip);
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
