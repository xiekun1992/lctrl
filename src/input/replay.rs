use std::{ffi::c_int, slice, thread};

use tracing::debug;

use crate::global::state::STATE;

use super::{
    listener::{KEY_DOWN, KEY_UP, MOUSE_DOWN, MOUSE_MOVE, MOUSE_REL_MOVE, MOUSE_UP, MOUSE_WHEEL},
    SERVER,
};

// #[link(name = "libcapture")]
extern "C" {
    fn mouse_init(left: c_int, top: c_int, right: c_int, bottom: c_int);
    fn mouse_move(x: c_int, y: c_int);
    fn mouse_wheel(direction: i32);
    fn mouse_down(button: i32);
    fn mouse_up(button: i32);

    fn keyboard_init();
    fn keydown(scancodes: *const c_int, len: c_int) -> c_int;
    fn keyup(scancodes: *const c_int, len: c_int) -> c_int;
}

static mut KEY_PRESSED: Vec<i32> = vec![];

fn replay_input(bytes: &[u32]) {
    unsafe {
        let bytes = slice::from_raw_parts(bytes.as_ptr() as *const i32, bytes.len());
        // debug!("{:?}", bytes);
        // println!("{} - {:?}", Local::now(), bytes);
        match bytes[0] {
            MOUSE_WHEEL => {
                // MouseWheel
                let delta = bytes[4];
                mouse_wheel(delta);
            }
            MOUSE_MOVE => {
                // MouseMove
                let x = bytes[1];
                let y = bytes[2];
                mouse_move(x, y);
            }
            MOUSE_DOWN => {
                // MouseDown
                mouse_down(bytes[3]);
            }
            MOUSE_UP => {
                // MouseUp
                mouse_up(bytes[3]);
            }
            KEY_DOWN => {
                debug!("{:?}", bytes);
                // keydown
                let scancode = bytes[2];
                let ctrl_key = bytes[3];
                let alt_key = bytes[4];
                let shift_key = bytes[5];
                let meta_key = bytes[6];
                let mut scancodes = Vec::new();
                scancodes.push(scancode);
                scancodes.push(ctrl_key);
                scancodes.push(alt_key);
                scancodes.push(shift_key);
                scancodes.push(meta_key);

                KEY_PRESSED.push(scancode);
                keydown(scancodes.as_ptr(), scancodes.len() as i32);
            }
            KEY_UP => {
                // keyup
                let scancode = bytes[2];
                let ctrl_key = bytes[3];
                let alt_key = bytes[4];
                let shift_key = bytes[5];
                let meta_key = bytes[6];
                let mut scancodes = Vec::new();
                scancodes.push(scancode);
                scancodes.push(ctrl_key);
                scancodes.push(alt_key);
                scancodes.push(shift_key);
                scancodes.push(meta_key);

                KEY_PRESSED.retain(|key_scancode| scancode != *key_scancode);
                debug!("{:?}", KEY_PRESSED);
                keyup(scancodes.as_ptr(), scancodes.len() as i32);
            }
            MOUSE_REL_MOVE => {
                // MouseMoveRelative
                let x = bytes[1];
                let y = bytes[2];
                mouse_move(x, y);
            }
            _ => {}
        }
    }
}

pub fn init() {
    thread::spawn(|| {
        match &STATE.lock() {
            Ok(state) => {
                let rect = state.screen_size.clone();
                println!("{:?}", rect);
                unsafe {
                    keyboard_init();
                    mouse_init(rect.left, rect.top, rect.right, rect.bottom);
                }
            }
            _ => {}
        }
        SERVER.recv(replay_input);
    });
}
