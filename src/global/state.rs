use std::{
    ffi::{c_int, c_long},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE};

use super::{
    db::DB,
    device::{DeviceInfo, RemoteDevice},
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug)]
#[repr(C)]
pub struct RECT {
    pub left: c_int,
    pub top: c_int,
    pub right: c_int,
    pub bottom: c_int,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
    pub fn from(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }
    pub fn to_arr(&self) -> [i32; 4] {
        [self.left, self.right, self.top, self.bottom]
    }
}

// #[link(name = "libcapture")]
extern "C" {
    fn get_screen_size() -> RECT;
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

pub struct State {
    pub remotes: Mutex<Vec<RemoteDevice>>,
    pub cur_device: DeviceInfo,
    pub remote_peer: Option<RemoteDevice>,
    pub screen_size: Rect,
    pub side: ControlSide,
    pub db: DB,
}

impl State {
    fn new() -> State {
        let db = DB::new();
        let mut screen_size = Rect::new();
        unsafe {
            let s = get_screen_size();
            let s = if s.left == s.right && s.top == s.bottom {
                match db.get_current_device() {
                    Some(size) => size,
                    None => RECT {
                        left: 0,
                        top: 0,
                        right: 800,
                        bottom: 600,
                    },
                }
            } else {
                s
            };
            info!("screen rect: {:?}", s);
            screen_size.left = s.left;
            screen_size.top = s.top;
            screen_size.right = s.right;
            screen_size.bottom = s.bottom;
        }
        let (remote_peer, side) = db.get_remote_peer();
        let remotes = Vec::new();
        // if let Some(peer) = remote_peer.clone() {
        //     remotes.push(peer);
        // }
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
        self.side = *side;
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
                        REMOTE_SCREEN_SIZE = rdev.screen_size.clone().to_arr();
                        SELF_SCREEN_SIZE = self.screen_size.clone().to_arr();
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
