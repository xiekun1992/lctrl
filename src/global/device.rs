use gethostname::gethostname;
use netdev::mac::MacAddr;
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, vec};

use super::state::{Rect, RECT};

extern "C" {
    fn get_screens(count: *mut i32) -> *const Rect;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub addr: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub mac_addr: String,
    pub broadcast_addr: Ipv4Addr,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceInfo {
    pub hostname: String,
    pub screens: Vec<Rect>,
    pub ifs: Vec<Interface>,
    pub auto_discover: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RemoteDevice {
    pub hostname: String,
    pub ip: String,
    pub netmask: String,
    pub mac_addr: String,
    pub screen_size: Rect,
    #[serde(default = "default_screens")]
    pub screens: Vec<Rect>,
    pub alive_timestamp: u64,
}

impl RemoteDevice {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or("".to_string())
    }
    pub fn from_json(json_str: String) -> RemoteDevice {
        // println!("remote device from json {}", json_str);
        let remote: RemoteDevice = serde_json::from_str(&json_str).unwrap_or(RemoteDevice {
            hostname: "".to_string(),
            ip: "".to_string(),
            netmask: "".to_string(),
            mac_addr: "".to_string(),
            screen_size: Rect {
                left: 0,
                top: 0,
                right: 800,
                bottom: 600,
            },
            screens: vec![],
            alive_timestamp: 0,
        });
        remote
    }
}

fn default_screens() -> Vec<Rect> {
    vec![Rect {
        top: 0,
        right: 1366,
        bottom: 768,
        left: 0,
    }]
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
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
        DeviceInfo {
            hostname: String::from(gethostname().to_str().unwrap_or("unknown hostname")),
            ifs: get_interfaces(),
            screens: screens.to_vec(),
            auto_discover: true,
        }
    }
}

pub fn get_interfaces() -> Vec<Interface> {
    let mut ifs = Vec::new();

    let interfaces = netdev::get_interfaces();
    for interface in interfaces {
        if !interface.is_up() {
            continue;
        }
        let mut avail = true;
        for ip in interface.ipv4.as_slice() {
            if ip.addr.is_link_local() || ip.addr.is_loopback() {
                avail = false;
            }
        }

        if avail {
            if let Some(ipv4) = interface.ipv4.get(0) {
                ifs.push(Interface {
                    addr: ipv4.addr,
                    netmask: ipv4.netmask,
                    mac_addr: interface.mac_addr.unwrap_or(MacAddr::zero()).to_string(),
                    broadcast_addr: calc_broadcast_addr(ipv4.addr, ipv4.netmask),
                });
            }
        }
    }

    ifs
}

pub fn calc_broadcast_addr(addr: Ipv4Addr, netmask: Ipv4Addr) -> Ipv4Addr {
    let ipv4 = addr.octets();
    let mask = netmask.octets();
    let mut broadcast = [0; 4];
    for idx in [0, 1, 2, 3] {
        broadcast[idx] = ipv4[idx] & mask[idx] | (mask[idx] ^ 0xff);
    }
    Ipv4Addr::new(broadcast[0], broadcast[1], broadcast[2], broadcast[3])
}

#[test]
fn calc_broadcast_addr_test() {
    let b = calc_broadcast_addr(
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(255, 255, 255, 0),
    );
    assert_eq!(b, Ipv4Addr::new(192, 168, 1, 255));

    let b1 = calc_broadcast_addr(
        Ipv4Addr::new(200, 222, 5, 100),
        Ipv4Addr::new(255, 128, 0, 0),
    );
    assert_eq!(b1, Ipv4Addr::new(200, 255, 255, 255));
}
