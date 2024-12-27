use serde::{Deserialize, Serialize};

use crate::global::device::RemoteDevice;

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteDevices {
    pub manual_remotes: Vec<RemoteDevice>,
    pub remotes: Vec<RemoteDevice>,
}
