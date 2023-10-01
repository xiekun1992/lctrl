pub mod udp_server;

use std::{sync::Arc, thread};
use crate::global::state::State;
use udp_server::UDPServer;

pub fn init(global_state: Arc<State>) {
  let udp_discover = UDPServer::new("0.0.0.0".to_string(), 1232);
  let server = Arc::new(udp_discover);

  let recv_state = Arc::clone(&global_state);
  let recv_server = Arc::clone(&server);
  thread::spawn(move || {
    recv_server.recv(recv_state);
  });
  
  let send_state = Arc::clone(&global_state);
  let send_server = Arc::clone(&server);
  thread::spawn(move || {
      send_server.send(send_state);
  });
}
