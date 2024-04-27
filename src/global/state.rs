use std::{
    ffi::c_int,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE};

use super::{
    db::DB,
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
    pub db: DB,
}

impl State {
    fn new() -> State {
        let mut screen_size = [0, 0];
        unsafe {
            get_screen_size(screen_size.as_mut_ptr());
        }
        let db = DB::new();
        let (remote_peer, side) = db.get_remote_peer();
        let mut remotes = Vec::new();
        if let Some(peer) = remote_peer.clone() {
            remotes.push(peer);
        }
        State {
            remotes: Mutex::new(remotes),
            cur_device: DeviceInfo::new(),
            screen_size,
            remote_peer,
            side,
            db,
        }
    }

    pub fn get_remote_peer(&self) -> Option<RemoteDevice> {
        self.remote_peer.clone()
    }

    pub fn set_remote_peer(&mut self, peer: Option<RemoteDevice>, side: &ControlSide) {
        match peer.clone() {
            Some(ref r) => {
                self.db.set_remote_peer(r, side);
            }
            None => {
                self.db.delete_remote_peer();
            }
        }
        self.remote_peer = peer;
    }

    pub fn get_remote(&self) -> Vec<RemoteDevice> {
        self.remotes.lock().unwrap().clone()
    }

    pub fn add_remote(&mut self, mut remote: RemoteDevice) {
        {
            let mut remotes = self.remotes.lock().unwrap();
            let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => {
                    println!("SystemTime before UNIX EPOCH!");
                    0
                }
            };
            let found = remotes.iter_mut().find(|item| item.ip.eq(&remote.ip));
            if found.is_none() {
                remote.alive_timestamp = timestamp;
                remotes.push(remote.clone());
            } else {
                found.unwrap().alive_timestamp = timestamp;
            }
            // println!("{:?}, {:?}", remotes, self.remote_peer);
        }
        match self.remote_peer.clone() {
            Some(rdev) => {
                if self.find_remote_by_ip(&rdev.ip).is_some() {
                    unsafe {
                        REMOTE_SCREEN_SIZE = rdev.screen_size.clone();
                        SELF_SCREEN_SIZE = self.screen_size.clone();
                        SIDE = self.side.clone();
                    }
                }
            }
            None => {}
        }
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

#[test]
fn test_time() {
    // 获取 UNIX 时间戳
    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    println!("Current timestamp: {}", timestamp);
}
