use crate::input::udp_server::UDPServer;
use lazy_static::lazy_static;
use tracing::info;

// pub mod clipboard;
pub mod listener;
pub mod replay;
mod udp_server;

lazy_static! {
    static ref SERVER: UDPServer = UDPServer::new(String::from("0.0.0.0"), 11233);
}

pub fn init() {
    // clipboard::init();
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    listener::init();
    replay::init();

    info!("input module init");
}
