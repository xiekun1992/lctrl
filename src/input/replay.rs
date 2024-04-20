use std::{ffi::c_int, mem, slice, thread};

use log::debug;

use super::SERVER;

#[link(name = "libinput")]
extern "C" {
    fn mouse_move(x: c_int, y: c_int);
    fn mouse_wheel(direction: i32);
    fn mouse_down(button: i32);
    fn mouse_up(button: i32);

    fn keyboard_init();
    fn keydown(scancodes: *const c_int, len: c_int) -> c_int;
    fn keyup(scancodes: *const c_int, len: c_int) -> c_int;
}

fn replay_input(bytes: &[u32]) {
    unsafe {
        let bytes = slice::from_raw_parts(bytes.as_ptr() as *const i32, bytes.len());
        debug!("{:?}", bytes);
        // println!("{} - {:?}", Local::now(), bytes);
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
                keydown(scancodes.as_ptr(), scancodes.len() as i32);
            }
            5 => {
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
                keyup(scancodes.as_ptr(), scancodes.len() as i32);
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

pub fn init() {
    thread::spawn(|| {
        unsafe {
            keyboard_init();
        }
        SERVER.recv(replay_input);
    });
}
