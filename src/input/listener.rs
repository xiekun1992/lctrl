use super::SERVER;
use crate::global::state::STATE;
use tracing::info;
// use tracing::debug;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_int, c_long},
    mem, slice, thread,
    time::Duration,
};

type CInputHandler = extern "C" fn(*const c_long);
type HotKeyHandler = extern "C" fn(*const [c_long; 7]);
// #[link(name = "libcapture")]
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
    TOP,
}

pub static mut REMOTE_SCREEN_SIZE: [f32; 4] = [0.0, 0.0, 0.0, 0.0]; // left, right, top, bottom
pub static mut SELF_SCREEN_SIZE: [i32; 4] = [0, 0, 0, 0]; // left, right, top, bottom
pub static mut SIDE: ControlSide = ControlSide::NONE;
static mut POS_IN_REMOTE_SCREEN: [f32; 2] = [0.0, 0.0];
// static mut POS_IN_REMOTE_SCREEN_FL: [f32; 2] = [0f32, 0f32];
static mut BLOCK: bool = false;
static mut MOUSE_BUTTON_HOLD: bool = false;
static mut IS_REMOTE_ALIVE: bool = false;

fn send_to_remote(ev: &[i32]) {
    unsafe {
        // println!("{:?}", ev);
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
        let ev = &[
            ev[0] as i32,
            ev[1] as i32,
            ev[2] as i32,
            ev[3] as i32,
            ev[4] as i32,
        ];
        // println!(
        //     "BLOCK={}, SIDE={:?}, POS_IN_REMOTE_SCREEN={:?}, mouse_type={}, x={}, y={}",
        //     BLOCK, SIDE, POS_IN_REMOTE_SCREEN, ev[0], ev[1], ev[2]
        // );
        // 控制状态下转发鼠标动作
        if BLOCK {
            // mousemoverel
            match ev[0] {
                MOUSE_REL_MOVE => {
                    let mut xfactor = 1.0;
                    let mut yfactor = 1.0;
                    if ev[1].abs() >= 3 {
                        xfactor *= 1.5;
                    }
                    if ev[2].abs() >= 3 {
                        yfactor *= 1.5;
                    }
                    if ev[1].abs() >= 10 {
                        xfactor *= 1.5;
                    }
                    if ev[2].abs() >= 10 {
                        yfactor *= 1.5;
                    }
                    if ev[1].abs() >= 20 {
                        xfactor *= 1.5;
                    }
                    if ev[2].abs() >= 20 {
                        yfactor *= 1.5;
                    }
                    POS_IN_REMOTE_SCREEN[0] += (ev[1] as f32) * xfactor;
                    POS_IN_REMOTE_SCREEN[1] += (ev[2] as f32) * yfactor;
                    // POS_IN_REMOTE_SCREEN[0] = POS_IN_REMOTE_SCREEN_FL[0] as i32;
                    // POS_IN_REMOTE_SCREEN[1] = POS_IN_REMOTE_SCREEN_FL[1] as i32;
                    // POS_IN_REMOTE_SCREEN[0] += ev[1];
                    // POS_IN_REMOTE_SCREEN[1] += ev[2];
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
                        ControlSide::TOP => {
                            if POS_IN_REMOTE_SCREEN[1] > REMOTE_SCREEN_SIZE[3] {
                                if !MOUSE_BUTTON_HOLD {
                                    listener_setBlock(0);
                                    BLOCK = false;
                                    POS_IN_REMOTE_SCREEN[1] = REMOTE_SCREEN_SIZE[3];
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
                    let x = POS_IN_REMOTE_SCREEN[0] as i32;
                    let y = POS_IN_REMOTE_SCREEN[1] as i32;
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
                                POS_IN_REMOTE_SCREEN[1] = ev[2] as f32;
                                BLOCK = true;
                            }
                        }
                        ControlSide::RIGHT => {
                            if ev[1] >= SELF_SCREEN_SIZE[1] {
                                listener_setBlock(1);
                                POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
                                POS_IN_REMOTE_SCREEN[1] = ev[2] as f32;
                                BLOCK = true;
                            }
                        }
                        ControlSide::TOP => {
                            if ev[2] <= SELF_SCREEN_SIZE[2] {
                                listener_setBlock(1);
                                POS_IN_REMOTE_SCREEN[0] = ev[1] as f32;
                                POS_IN_REMOTE_SCREEN[1] = REMOTE_SCREEN_SIZE[3];
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
            let ev = &[
                ev[0] as i32,
                ev[1] as i32,
                ev[2] as i32,
                ev[3] as i32,
                ev[4] as i32,
                ev[5] as i32,
                ev[6] as i32,
            ];
            // debug!("keyboard: {:?}", ev);
            send_to_remote(ev);
        }
    }
}

extern "C" fn hotkey_handler(hotkeys: *const [c_long; 7]) {
    info!("unblock hotkey triggered");
    thread::sleep(Duration::from_millis(100)); // 让控制端有时间处理完按键释放事件，防止热建触发时按键还按着
    unsafe {
        if BLOCK {
            BLOCK = false;
            listener_setBlock(0);
            POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
            let center_x = (SELF_SCREEN_SIZE[1] - SELF_SCREEN_SIZE[0]) / 2;
            let center_y = (SELF_SCREEN_SIZE[3] - SELF_SCREEN_SIZE[2]) / 2;
            mouse_move(center_x, center_y);
        } else {
            BLOCK = true;
            listener_setBlock(1);
            POS_IN_REMOTE_SCREEN[0] = (REMOTE_SCREEN_SIZE[1] - REMOTE_SCREEN_SIZE[0]) / 2.0;
            POS_IN_REMOTE_SCREEN[1] = (REMOTE_SCREEN_SIZE[3] - REMOTE_SCREEN_SIZE[2]) / 2.0;

            // 鼠标相对移动转换成绝对移动
            let x = POS_IN_REMOTE_SCREEN[0] as i32;
            let y = POS_IN_REMOTE_SCREEN[1] as i32;
            let bytes_to_send = [1, x, y];
            send_to_remote(bytes_to_send.as_slice());
        }
        // 通知受控端将按键释放
        let hotkeys = slice::from_raw_parts(hotkeys, 2);
        for key in hotkeys {
            let i32_key = [
                key[0] as i32,
                key[1] as i32,
                key[2] as i32,
                key[3] as i32,
                key[4] as i32,
                key[5] as i32,
                key[6] as i32,
            ];
            send_to_remote(i32_key.as_slice());
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
            BLOCK = false;
            IS_REMOTE_ALIVE = false;
            POS_IN_REMOTE_SCREEN[0] = REMOTE_SCREEN_SIZE[0];
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
