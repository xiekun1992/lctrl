use std::{
    ffi::{c_char, c_int, c_long, c_uchar, c_ushort, CStr, OsStr, OsString},
    iter::once,
    mem,
    os::windows::prelude::{OsStrExt, OsStringExt},
    slice, thread,
    time::Duration,
};

use crate::{global::state::STATE, input::udp_server::UDPServer};

type CInputHandler = extern "C" fn(*const c_long);
#[link(name = "libcapture")]
extern "C" {

    fn listener_init(mouseHanlder: CInputHandler, keyboardHanlder: CInputHandler);
    fn listener_listen();
    // fn listener_dispose();
    // fn listener_close();
    fn listener_setBlock(block: c_int);
}

#[link(name = "libinput")]
extern "C" {
    fn mouse_move(x: c_int, y: c_int);
    fn mouse_wheel(direction: i32);
    fn mouse_down(button: i32);
    fn mouse_up(button: i32);

    fn keyboard_init();
    // fn keyboard_dispose();
    fn keydown(scancode: c_int) -> c_int;
    fn keyup(scancode: c_int) -> c_int;
    fn scancode_to_keycode(scancode: c_int) -> c_int;
}
use chrono::Local;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVER: UDPServer = UDPServer::new(String::from("0.0.0.0"), 1233);
}

fn send_to_remote(ev: *const c_long, num_of_elements: usize) {
    unsafe {
        let ev = slice::from_raw_parts(ev, num_of_elements);
        let bytes =
            slice::from_raw_parts(ev.as_ptr() as *const u8, ev.len() * mem::size_of::<i32>());
        // println!("{:?}", bytes);
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
        println!("{} - {:?}", Local::now(), bytes);
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
                let scancode = bytes[2];
                // keyboard_init();
                keydown(scancode);
            }
            5 => {
                // keyup
                let scancode = bytes[2];
                keyup(scancode);
            }
            6 => {
                // MouseMoveRelative
                let x = bytes[1];
                let y = bytes[2];
                mouse_move(x, y);
            }
            _ => {}
        }
    }
}

extern "C" fn mouse_handler(ev: *const c_long) {
    send_to_remote(ev, 5);
    println!("mouse");
}

extern "C" fn keyboard_handler(ev: *const c_long) {
    send_to_remote(ev, 3);
    println!("keyboard");
}

pub fn init() {
    thread::spawn(|| {
        unsafe {
            listener_init(mouse_handler, keyboard_handler);
            listener_listen();
        }
    });
    thread::spawn(|| {
        unsafe {
            keyboard_init();
        }
        SERVER.recv(cb);
    });
}
