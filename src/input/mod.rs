mod udp_server;

mod clipboard;
pub mod listener;

pub fn init() {
    listener::init();
}
