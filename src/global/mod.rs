use std::{thread, time::Duration};

use tracing::info;

use self::{device::get_interfaces, state::STATE};

pub mod db;
pub mod device;
pub mod setting;
pub mod state;

pub fn init() {
    thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(1));

        let ifs = get_interfaces();
        {
            match STATE.lock() {
                Ok(mut state) => {
                    if state.cur_device.ifs.len() != ifs.len() {
                        state.cur_device.ifs = ifs;
                    } else if state.cur_device.ifs != ifs {
                        state.cur_device.ifs = ifs;
                    }
                }
                Err(_err) => {}
            }
        }
    });
}
