use std::{
    ffi::{c_int, c_long},
    mem, slice, thread,
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
    // fn scancode_to_keycode(scancode: c_int) -> c_int;
}
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum ControlSide {
    NONE,
    LEFT,
    RIGHT,
}

lazy_static! {
    static ref SERVER: UDPServer = UDPServer::new(String::from("0.0.0.0"), 11233);
}
pub static mut REMOTE_SCREEN_SIZE: [i32; 2] = [0, 0];
pub static mut SELF_SCREEN_SIZE: [i32; 2] = [0, 0];
pub static mut SIDE: ControlSide = ControlSide::NONE;
static mut POS_IN_REMOTE_SCREEN: [i32; 2] = [0, 0];
static mut BLOCK: bool = false;

fn send_to_remote(ev: &[i32]) {
    unsafe {
        let bytes =
            slice::from_raw_parts(ev.as_ptr() as *const u8, ev.len() * mem::size_of::<i32>());
        println!("{:?}", bytes);
        {
            match &STATE.lock().unwrap().get_remote_peer() {
                Some(peer) => {
                    let addr = format!("{}:11233", peer.ip);
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
    unsafe {
        let ev = slice::from_raw_parts(ev, 5);

        // println!(
        //     "BLOCK={}, SIDE={:?}, POS_IN_REMOTE_SCREEN={:?}, mouse_type={}, x={}, y={}",
        //     BLOCK, SIDE, POS_IN_REMOTE_SCREEN, ev[0], ev[1], ev[2]
        // );

        if BLOCK {
            // mousemoverel
            if ev[0] == 6 {
                POS_IN_REMOTE_SCREEN[0] += ev[1];
                POS_IN_REMOTE_SCREEN[1] += ev[2];

                match SIDE {
                    ControlSide::LEFT => {
                        if POS_IN_REMOTE_SCREEN[0] > REMOTE_SCREEN_SIZE[0] {
                            listener_setBlock(0);
                            BLOCK = false;
                            POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
                        }
                    }
                    ControlSide::RIGHT => {
                        if POS_IN_REMOTE_SCREEN[0] < 0 {
                            listener_setBlock(0);
                            BLOCK = false;
                            POS_IN_REMOTE_SCREEN[0] = 0;
                        }
                    }
                    _ => {}
                }
                if POS_IN_REMOTE_SCREEN[0] < 0 {
                    POS_IN_REMOTE_SCREEN[0] = 0;
                }
                if POS_IN_REMOTE_SCREEN[0] > REMOTE_SCREEN_SIZE[0] {
                    POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
                }
                if POS_IN_REMOTE_SCREEN[1] < 0 {
                    POS_IN_REMOTE_SCREEN[1] = 0;
                }
                if POS_IN_REMOTE_SCREEN[1] > REMOTE_SCREEN_SIZE[1] {
                    POS_IN_REMOTE_SCREEN[1] = REMOTE_SCREEN_SIZE[1];
                }
                let x = POS_IN_REMOTE_SCREEN[0]; //(POS_IN_REMOTE_SCREEN[0] as f32 / SCREEN_SIZE[0] as f32 * 1366.0) as i32;
                let y = POS_IN_REMOTE_SCREEN[1]; //POS_IN_REMOTE_SCREEN[1] as f32 / SCREEN_SIZE[1] as f32 * 768.0) as i32;
                                                 // println!(
                                                 //     "BLOCK={}, POS_IN_REMOTE_SCREEN={}, SCREEN_SIZE={}, x={}, y={}",
                                                 //     BLOCK, POS_IN_REMOTE_SCREEN[1], REMOTE_SCREEN_SIZE[1], x, y
                                                 // );
                let bytes_to_send = [1, x, y];
                send_to_remote(bytes_to_send.as_slice());
            } else if ev[0] != 1 {
                send_to_remote(ev);
            }
        }

        if !BLOCK && ev[0] == 1 {
            // mousemove
            match SIDE {
                ControlSide::LEFT => {
                    if ev[1] <= 0 {
                        listener_setBlock(1);
                        POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
                        POS_IN_REMOTE_SCREEN[1] = ev[2];
                        BLOCK = true;
                    }
                }
                ControlSide::RIGHT => {
                    if ev[1] >= SELF_SCREEN_SIZE[0] {
                        listener_setBlock(1);
                        POS_IN_REMOTE_SCREEN[0] = 0;
                        POS_IN_REMOTE_SCREEN[1] = ev[2];
                        BLOCK = true;
                    }
                }
                _ => {}
            }
        }
    }
}

extern "C" fn keyboard_handler(ev: *const c_long) {
    unsafe {
        if BLOCK {
            let ev = slice::from_raw_parts(ev, 3);
            send_to_remote(ev);
        }
    }
}

pub fn init() {
    // unsafe {
    //     get_screen_size(SCREEN_SIZE.as_mut_ptr());
    // }

    thread::spawn(|| unsafe {
        listener_init(mouse_handler, keyboard_handler);
        listener_listen();
    });
    thread::spawn(|| {
        unsafe {
            keyboard_init();
        }
        SERVER.recv(cb);
    });
}
