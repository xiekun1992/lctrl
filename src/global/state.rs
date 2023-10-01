use std::sync::Mutex;

use super::device::{RemoteDevice, DeviceInfo};

pub struct State {
  remotes: Mutex<Vec<RemoteDevice>>,
  pub cur_device: DeviceInfo
}

impl State {
  pub fn new() -> State {
    State {
      remotes: Mutex::new(Vec::new()),
      cur_device: DeviceInfo::new()
    }
  }

  pub fn add_remote(&self, remote: RemoteDevice) {
    let mut remotes = self.remotes.lock().unwrap();
    if !remotes.contains(&remote) {
      remotes.push(remote);
    }
  }
}