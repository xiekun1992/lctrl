use serde::{Deserialize, Serialize};

use crate::{global::device::RemoteDevice, input::listener::ControlSide};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteDevices {
    pub manual_remotes: Vec<RemoteDevice>,
    pub remotes: Vec<RemoteDevice>,
}

impl RemoteDevices {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScreenSetting {
    pub vcoord: Vec<EdgeSetting>,
    pub hcoord: Vec<EdgeSetting>,
}
impl ScreenSetting {
    pub fn new() -> Self {
        ScreenSetting {
            vcoord: vec![],
            hcoord: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EdgeSetting {
    pub screen: u8,
    pub start: f64,
    pub end: f64,
}
