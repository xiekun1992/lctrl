use std::thread;

mod udp_server;

pub mod listener;
mod clipboard;

pub fn init() {
    listener::init();
    // thread::spawn(|| {
    //     listener::init();
    // });
    // thread::spawn(|| {
    //     clipboard::init();
    // });
}
