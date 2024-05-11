use super::SERVER;
use crate::global::state::STATE;
use log::info;
// use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_int, c_long},
    mem, slice, thread,
};

type CInputHandler = extern "C" fn(*const c_long);
type HotKeyHandler = extern "C" fn();
#[link(name = "libcapture")]
extern "C" {
    fn listener_init(
        mouseHanlder: CInputHandler,
        keyboardHanlder: CInputHandler,
        hotkeyHandler: HotKeyHandler,
    );
    fn listener_listen();
    fn listener_setBlock(block: c_int);
    fn mouse_move(x: c_int, y: c_int);
}

pub const MOUSE_WHEEL: i32 = 0;
pub const MOUSE_MOVE: i32 = 1;
pub const MOUSE_DOWN: i32 = 2;
pub const MOUSE_UP: i32 = 3;
pub const KEY_DOWN: i32 = 4;
pub const KEY_UP: i32 = 5;
pub const MOUSE_REL_MOVE: i32 = 6;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum ControlSide {
    NONE,
    LEFT,
    RIGHT,
}

pub static mut REMOTE_SCREEN_SIZE: [i32; 4] = [0, 0, 0, 0]; // left, right, top, bottom
pub static mut SELF_SCREEN_SIZE: [i32; 4] = [0, 0, 0, 0]; // left, right, top, bottom
pub static mut SIDE: ControlSide = ControlSide::NONE;
static mut POS_IN_REMOTE_SCREEN: [i32; 2] = [0, 0];
static mut BLOCK: bool = false;
static mut MOUSE_BUTTON_HOLD: bool = false;
static mut IS_REMOTE_ALIVE: bool = false;

fn send_to_remote(ev: &[i32]) {
    unsafe {
        let bytes =
            slice::from_raw_parts(ev.as_ptr() as *const u8, ev.len() * mem::size_of::<i32>());
        // println!("{:?}", bytes);
        {
            match &STATE.lock().unwrap().get_remote_peer() {
                Some(peer) => {
                    let addr = format!("{}:11233", peer.ip);
                    // debug!("{:?}", ev);
                    SERVER.send(&bytes, &addr);
                }
                None => {}
            }
        }
    }
}

extern "C" fn mouse_handler(ev: *const c_long) {
    unsafe {
        if !IS_REMOTE_ALIVE {
            return;
        }
        let ev = slice::from_raw_parts(ev, 5);

        // println!(
        //     "BLOCK={}, SIDE={:?}, POS_IN_REMOTE_SCREEN={:?}, mouse_type={}, x={}, y={}",
        //     BLOCK, SIDE, POS_IN_REMOTE_SCREEN, ev[0], ev[1], ev[2]
        // );
        // 控制状态下转发鼠标动作
        if BLOCK {
            // mousemoverel
            match ev[0] {
                MOUSE_REL_MOVE => {
                    POS_IN_REMOTE_SCREEN[0] += ev[1];
                    POS_IN_REMOTE_SCREEN[1] += ev[2];
                    // 检测是否移动到屏幕边缘并解除控制
                    match SIDE {
                        ControlSide::LEFT => {
                            if POS_IN_REMOTE_SCREEN[0] > REMOTE_SCREEN_SIZE[1] {
                                if !MOUSE_BUTTON_HOLD {
                                    listener_setBlock(0);
                                    BLOCK = false;
                                    POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[1];
                                }
                            }
                        }
                        ControlSide::RIGHT => {
                            if POS_IN_REMOTE_SCREEN[0] < REMOTE_SCREEN_SIZE[0] {
                                if !MOUSE_BUTTON_HOLD {
                                    listener_setBlock(0);
                                    BLOCK = false;
                                    POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
                                }
                            }
                        }
                        _ => {}
                    }
                    // 检测是否超过屏幕上下限
                    if POS_IN_REMOTE_SCREEN[0] < REMOTE_SCREEN_SIZE[0] {
                        POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
                    }
                    if POS_IN_REMOTE_SCREEN[0] > REMOTE_SCREEN_SIZE[1] {
                        POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[1];
                    }
                    if POS_IN_REMOTE_SCREEN[1] < REMOTE_SCREEN_SIZE[2] {
                        POS_IN_REMOTE_SCREEN[1] = REMOTE_SCREEN_SIZE[2];
                    }
                    if POS_IN_REMOTE_SCREEN[1] > REMOTE_SCREEN_SIZE[3] {
                        POS_IN_REMOTE_SCREEN[1] = REMOTE_SCREEN_SIZE[3];
                    }

                    // 鼠标相对移动转换成绝对移动
                    let x = POS_IN_REMOTE_SCREEN[0];
                    let y = POS_IN_REMOTE_SCREEN[1];
                    let bytes_to_send = [1, x, y];
                    send_to_remote(bytes_to_send.as_slice());
                }
                MOUSE_MOVE => {}
                MOUSE_DOWN => {
                    MOUSE_BUTTON_HOLD = true;
                    send_to_remote(ev);
                }
                MOUSE_UP => {
                    MOUSE_BUTTON_HOLD = false;
                    send_to_remote(ev);
                }
                _ => {
                    send_to_remote(ev);
                }
            }
        }

        // 非控制下检测鼠标移动，判断是否进入控制
        if !BLOCK {
            match ev[0] {
                MOUSE_MOVE => {
                    // mousemove
                    if MOUSE_BUTTON_HOLD {
                        return;
                    }
                    match SIDE {
                        ControlSide::LEFT => {
                            if ev[1] <= SELF_SCREEN_SIZE[0] {
                                listener_setBlock(1);
                                POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[1];
                                POS_IN_REMOTE_SCREEN[1] = ev[2];
                                BLOCK = true;
                            }
                        }
                        ControlSide::RIGHT => {
                            if ev[1] >= SELF_SCREEN_SIZE[1] {
                                listener_setBlock(1);
                                POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
                                POS_IN_REMOTE_SCREEN[1] = ev[2];
                                BLOCK = true;
                            }
                        }
                        _ => {}
                    }
                }
                MOUSE_DOWN => {
                    MOUSE_BUTTON_HOLD = true;
                }
                MOUSE_UP => {
                    MOUSE_BUTTON_HOLD = false;
                }
                _ => {}
            }
        }
    }
}

extern "C" fn keyboard_handler(ev: *const c_long) {
    unsafe {
        if !IS_REMOTE_ALIVE {
            return;
        }
        if BLOCK {
            let ev = slice::from_raw_parts(ev, 7);
            // debug!("keyboard: {:?}", ev);
            send_to_remote(ev);
        }
    }
}

extern "C" fn hotkey_handler() {
    info!("unblock hotkey triggered");
    unsafe {
        if BLOCK {
            BLOCK = false;
            listener_setBlock(0);
            POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
            let center_x = (SELF_SCREEN_SIZE[1] - SELF_SCREEN_SIZE[0]) / 2;
            let center_y = (SELF_SCREEN_SIZE[3] - SELF_SCREEN_SIZE[2]) / 2;
            mouse_move(center_x, center_y);
        }
    }
}

pub fn init() {
    thread::spawn(|| unsafe {
        listener_init(mouse_handler, keyboard_handler, hotkey_handler);
        listener_listen();
    });
}

pub fn release() {
    // debug!("release");
    unsafe {
        if IS_REMOTE_ALIVE {
            // REMOTE_SCREEN_SIZE = [0, 0];
            // SELF_SCREEN_SIZE = [0, 0];
            // SIDE = ControlSide::NONE;
            BLOCK = false;
            IS_REMOTE_ALIVE = false;
            // POS_IN_REMOTE_SCREEN[0] = 0;
            listener_setBlock(0);
        }
    }
}

pub fn keepalive() {
    // debug!("keepalive");
    unsafe {
        IS_REMOTE_ALIVE = true;
    }
}
