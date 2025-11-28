use std::{
    ffi::{c_int, c_long},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE},
    web_api::dto::ScreenSetting,
};

use super::{
    db::DB,
    device::{DeviceInfo, RemoteDevice},
    setting::Setting,
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
#[repr(C)]
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
    pub fn to_float_arr(&self) -> [f32; 4] {
        [
            self.left as f32,
            self.right as f32,
            self.top as f32,
            self.bottom as f32,
        ]
    }
}

// #[link(name = "libcapture")]
extern "C" {
    fn get_screen_size() -> RECT;
    fn get_screens(count: *mut i32) -> *const Rect;
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

pub struct State {
    pub remotes: Mutex<Vec<RemoteDevice>>,
    pub manual_remotes: Mutex<Vec<RemoteDevice>>,
    pub cur_device: DeviceInfo,
    pub remote_peer: Option<RemoteDevice>,
    pub screen_size: Rect,
    pub screens: Vec<Rect>,
    pub side: ControlSide,
    pub db: DB,
    pub setting: Setting,
}

impl State {
    fn new() -> State {
        let db = DB::new();
        db.initialize();

        let mut screen_size = Rect::new();
        let s = unsafe { get_screen_size() };
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

        let mut count = 0;
        let zero_screens = vec![];
        let screens = unsafe {
            let screens_rects = get_screens(&mut count);
            if screens_rects.is_null() || count == 0 {
                zero_screens.as_slice()
            } else {
                std::slice::from_raw_parts(screens_rects, count as usize)
            }
        };
        let screens = if count == 0 {
            db.get_screens()
        } else {
            screens.to_vec()
        };
        info!("screens: {:#?}, count: {:?}", screens, count);
        let (remote_peer, side) = db.get_remote_peer();
        let remotes = Vec::new();
        let manual_remotes = Vec::new();
        // if let Some(peer) = remote_peer.clone() {
        //     remotes.push(peer);
        // }
        let mut cur_device = DeviceInfo::new();
        cur_device.screens = screens.clone();

        let setting = db.get_setting();
        State {
            remotes: Mutex::new(remotes),
            manual_remotes: Mutex::new(manual_remotes),
            cur_device,
            screen_size,
            screens,
            remote_peer,
            side,
            db,
            setting,
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
        if let Ok(remotes) = self.remotes.lock() {
            remotes.clone()
        } else {
            vec![]
        }
    }
    pub fn get_manual_remote(&self) -> Vec<RemoteDevice> {
        if let Ok(manual_remotes) = self.manual_remotes.lock() {
            manual_remotes.clone()
        } else {
            vec![]
        }
    }

    pub fn add_manual_remote(&mut self, manual_remote: RemoteDevice) {
        match self.manual_remotes.try_lock() {
            Ok(mut r) => {
                let found = r.iter().find(|item| item.ip.eq(&manual_remote.ip));
                if found.is_none() {
                    if let Ok(mut remotes) = self.remotes.try_lock() {
                        remotes.retain(|item| item.ip.ne(&manual_remote.ip));
                    }
                    r.push(manual_remote);
                }
            }
            Err(_e) => {}
        }
    }

    pub fn add_remote(&mut self, mut remote: RemoteDevice) {
        match self.manual_remotes.lock() {
            Ok(remotes) => {
                if remotes.iter().find(|item| item.ip.eq(&remote.ip)).is_some() {
                    return;
                }
            }
            Err(_e) => {}
        }
        match self.remotes.lock() {
            Ok(mut remotes) => {
                let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => duration.as_secs(),
                    Err(_) => {
                        println!("SystemTime before UNIX EPOCH!");
                        0
                    }
                };
                let found = remotes.iter_mut().find(|item| item.ip.eq(&remote.ip));
                match found {
                    Some(r) => {
                        r.alive_timestamp = timestamp;
                    }
                    None => {
                        remote.alive_timestamp = timestamp;
                        remotes.push(remote.clone());
                    }
                }
            }
            Err(_e) => {}
        }
        match self.remote_peer.clone() {
            Some(rdev) => {
                if self.find_remote_by_ip(&rdev.ip).is_some() {
                    unsafe {
                        REMOTE_SCREEN_SIZE = rdev.screen_size.clone().to_float_arr();
                        SELF_SCREEN_SIZE = self.screen_size.clone().to_arr();
                        SIDE = self.side.clone();
                    }
                }
            }
            None => {}
        }
    }

    pub fn del_remote(&self, ip: String) {
        if let Ok(mut remotes) = self.remotes.lock() {
            remotes.retain(|item| item.ip != ip);
        }
    }

    pub fn find_remote_by_ip(&self, ip: &str) -> Option<RemoteDevice> {
        let res = match self.remotes.lock() {
            Ok(remotes) => remotes.iter().find(|item| item.ip.as_str().eq(ip)).cloned(),
            Err(_e) => None,
        };
        if res.is_some() {
            return res;
        }
        let res = match self.manual_remotes.lock() {
            Ok(remotes) => remotes.iter().find(|item| item.ip.as_str().eq(ip)).cloned(),
            Err(_e) => None,
        };
        return res;
    }

    pub fn get_setting(&self) -> Setting {
        self.setting.clone()
    }

    pub fn set_setting(&mut self, setting: Setting) {
        self.db.set_setting(&setting);
        self.setting = setting;
    }

    pub fn set_auto_discover(&mut self, active: bool) {
        self.setting.auto_discover = active;
    }

    pub fn set_screens_setting(&mut self, screen_setting: ScreenSetting) {
        self.setting.screen_setting = screen_setting;
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
