use std::thread;

mod udp_server;

mod listener;
mod clipboard;

pub fn init() {
    thread::spawn(|| {
        listener::init();
    });
    // thread::spawn(|| {
    //     clipboard::init();
    // });
}
