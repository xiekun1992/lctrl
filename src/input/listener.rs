use std::{
    ffi::{c_char, c_int, c_long, c_uchar, c_ushort, CStr, OsStr, OsString},
    iter::once,
    mem,
    os::windows::prelude::{OsStrExt, OsStringExt},
    slice, thread,
    time::Duration,
};

use crate::{global::state::STATE, input::udp_server::UDPServer};

#[repr(C)]
// #[no_mangle]
struct Square {
    a: c_int,
    b: c_int,
}
#[repr(C)]
enum MouseButton {
    MouseLeft = 1,
    MouseMiddle,
    MouseRight,
}
#[repr(C)]
enum MouseWheel {
    WheelUp = -1,
    WheelDown = 1,
}
enum MouseType {
    MouseWheel = 0,
    MouseMove,
    MouseDown,
    MouseUp,
    KeyDown,
    KeyUp,
}
type CInputHandler = extern "C" fn(*const c_long);
#[link(name = "libcapture")]
extern "C" {
    fn mouse_move(x: c_int, y: c_int);
    fn mouse_wheel(direction: i32);
    fn mouse_down(button: i32);
    fn mouse_up(button: i32);

    fn listener_init(mouseHanlder: CInputHandler, keyboardHanlder: CInputHandler);
    fn listener_listen();
    fn charToKeycode(scancode: c_int) -> c_int;
}
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVER: UDPServer = UDPServer::new(String::from("0.0.0.0"), 1233);
}
struct MouseInput {
    flag: i32,
    x: i32,
    y: i32,
    button: i32,
    delta: i32,
}

fn send_to_remote(ev: *const c_long, num_of_elements: usize) {
    unsafe {
        let ev = slice::from_raw_parts(ev, num_of_elements);
        let bytes =
            slice::from_raw_parts(ev.as_ptr() as *const u8, ev.len() * mem::size_of::<i32>());
        println!("{:?}", bytes);
        {
            match &STATE.lock().unwrap().remote_peer {
                Some(peer) => {
                    let addr = format!("{}:1233", peer.ip);
                    SERVER.send(&bytes, &addr);
                }
                None => {}
            }
        }
    }
}

fn cb(bytes: &[u8]) {
    unsafe {
        let bytes = slice::from_raw_parts(
            bytes.as_ptr() as *const i32,
            bytes.len() * mem::size_of::<u8>(),
        );
        match bytes[0] {
            0 => {
                // MouseWheel
                let delta = bytes[4];
                mouse_wheel(delta);
            }
            1 => {
                // MouseMove
                let x = bytes[1];
                let y = bytes[2];
                mouse_move(x, y);
            }
            2 => {
                // MouseDown
                mouse_down(bytes[3]);
            }
            3 => {
                // MouseUp
                mouse_up(bytes[3]);
            }
            4 => {
                // keydown
                let vkcode = bytes[1];
                let scancode = bytes[2];
                
            }
            5 => {
                // keyup
                let vkcode = bytes[1];
                let scancode = bytes[2];
            }
            _ => {}
        }
    }
}

extern "C" fn mouse_handler(ev: *const c_long) {
    send_to_remote(ev, 5);
}

extern "C" fn keyboard_handler(ev: *const c_long) {
    send_to_remote(ev, 3);
}

pub fn init() {
    thread::spawn(|| {
        SERVER.recv(cb);
    });

    unsafe {
        listener_init(mouse_handler, keyboard_handler);
        listener_listen();
    }
}
