use super::SERVER;
use crate::global::{STATE, get_listener_state, get_is_remote_alive, get_block, set_block, set_is_remote_alive, set_pos_in_remote_screen, set_mouse_button_hold, get_mouse_button_hold};
use tracing::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_int, c_long},
    mem, slice, thread,
    time::Duration,
};

#[cfg(any(target_os = "macos"))]
type InputType = c_int;

#[cfg(any(target_os = "windows", target_os = "linux"))]
type InputType = c_long;

type CInputHandler = extern "C" fn(*const InputType);
type HotKeyHandler = extern "C" fn(*const [InputType; 7]);
// #[link(name = "libcapture")]
extern "C" {
    fn listener_init(
        mouseHanlder: CInputHandler,
        keyboardHanlder: CInputHandler,
        hotkeyHandler: HotKeyHandler,
    );
    fn listener_listen();
    fn listener_setBlock(block: c_int);
    // fn mouse_move(x: c_int, y: c_int);
    #[cfg(target_os = "macos")]
    fn power_set_replay_prevent(prevent: c_int);
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

/// Listener state - encapsulates all state variables
/// Defined here, managed in global/mod.rs
pub struct ListenerState {
    pub remote_screen_size: [f32; 4],  // left, right, top, bottom
    pub self_screen_size: [i32; 4],    // left, right, top, bottom
    pub side: ControlSide,
    pub pos_in_remote_screen: [f32; 2],
    pub block: bool,
    pub mouse_button_hold: bool,
    pub is_remote_alive: bool,
}

impl ListenerState {
    pub fn new() -> Self {
        Self {
            remote_screen_size: [0.0, 0.0, 0.0, 0.0],
            self_screen_size: [0, 0, 0, 0],
            side: ControlSide::NONE,
            pos_in_remote_screen: [0.0, 0.0],
            block: false,
            mouse_button_hold: false,
            is_remote_alive: false,
        }
    }
}

fn send_to_remote(ev: &[i32]) {
    // println!("{:?}", ev);
    let bytes = unsafe {
        slice::from_raw_parts(ev.as_ptr() as *const u8, ev.len() * mem::size_of::<i32>())
    };
    // println!("{:?}", bytes);
    
    let addr = match STATE.lock() {
            Ok(s) => match s.get_remote_peer() {
                Some(peer) => format!("{}:11233", peer.ip),
                None => return
            },
            Err(_e) => {
                error!("send_to_remote state lock failed");
                return;
            }
        };
    if let Err(e) = SERVER.send(&bytes, &addr) {
        error!("send input to {} failed: {}", addr, e);
        release();
    }
}

extern "C" fn mouse_handler(ev: *const InputType) {
    let ev = unsafe { slice::from_raw_parts(ev, 5) };
    let ev = &[
        ev[0] as i32,
        ev[1] as i32,
        ev[2] as i32,
        ev[3] as i32,
        ev[4] as i32,
    ];

    if !get_is_remote_alive() {
        return;
    }

    if let Ok(mut state) = get_listener_state().lock() {
        // Control mode: forward mouse actions
        if state.block {
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
                    state.pos_in_remote_screen[0] += (ev[1] as f32) * xfactor;
                    state.pos_in_remote_screen[1] += (ev[2] as f32) * yfactor;

                    // Check if moved to screen edge and release control
                    match state.side {
                        ControlSide::LEFT => {
                            if state.pos_in_remote_screen[0] > state.remote_screen_size[1] {
                                if !state.mouse_button_hold {
                                    unsafe { listener_setBlock(0); }
                                    state.block = false;
                                    state.pos_in_remote_screen[0] = state.remote_screen_size[1];
                                }
                            }
                        }
                        ControlSide::RIGHT => {
                            if state.pos_in_remote_screen[0] < state.remote_screen_size[0] {
                                if !state.mouse_button_hold {
                                    unsafe { listener_setBlock(0); }
                                    state.block = false;
                                    state.pos_in_remote_screen[0] = state.remote_screen_size[0];
                                }
                            }
                        }
                        ControlSide::TOP => {
                            if state.pos_in_remote_screen[1] > state.remote_screen_size[3] {
                                if !state.mouse_button_hold {
                                    unsafe { listener_setBlock(0); }
                                    state.block = false;
                                    state.pos_in_remote_screen[1] = state.remote_screen_size[3];
                                }
                            }
                        }
                        _ => {}
                    }

                    // Check if exceeds screen bounds
                    if state.pos_in_remote_screen[0] < state.remote_screen_size[0] {
                        state.pos_in_remote_screen[0] = state.remote_screen_size[0];
                    }
                    if state.pos_in_remote_screen[0] > state.remote_screen_size[1] {
                        state.pos_in_remote_screen[0] = state.remote_screen_size[1];
                    }
                    if state.pos_in_remote_screen[1] < state.remote_screen_size[2] {
                        state.pos_in_remote_screen[1] = state.remote_screen_size[2];
                    }
                    if state.pos_in_remote_screen[1] > state.remote_screen_size[3] {
                        state.pos_in_remote_screen[1] = state.remote_screen_size[3];
                    }

                    // Convert relative mouse movement to absolute
                    let x = state.pos_in_remote_screen[0] as i32;
                    let y = state.pos_in_remote_screen[1] as i32;
                    let bytes_to_send = [1, x, y];
                    send_to_remote(bytes_to_send.as_slice());
                }
                MOUSE_MOVE => {}
                MOUSE_DOWN => {
                    state.mouse_button_hold = true;
                    send_to_remote(ev);
                }
                MOUSE_UP => {
                    state.mouse_button_hold = false;
                    send_to_remote(ev);
                }
                MOUSE_WHEEL => {
                    send_to_remote(ev);
                }
                _ => {
                    send_to_remote(ev);
                }
            }
            return;
        }
    }

    // Non-control mode: detect mouse movement to enter control
    if !get_block() {
        match ev[0] {
            MOUSE_MOVE => {
                if get_mouse_button_hold() {
                    return;
                }
                
                if let Ok(state) = get_listener_state().lock() {
                    match state.side {
                        ControlSide::LEFT => {
                            if ev[1] <= state.self_screen_size[0] {
                                let enable_control = {
                                    if let Ok(s) = STATE.lock() {
                                        s.get_setting().enable_control && s.get_setting().cursor_across_screens
                                    } else {
                                        false
                                    }
                                };
                                if !enable_control {
                                    return;
                                }

                                unsafe { listener_setBlock(1); }
                                set_pos_in_remote_screen([state.remote_screen_size[1], ev[2] as f32]);
                                set_block(true);
                            }
                        }
                        ControlSide::RIGHT => {
                            if ev[1] >= state.self_screen_size[1] {
                                let enable_control = {
                                    if let Ok(s) = STATE.lock() {
                                        s.get_setting().enable_control && s.get_setting().cursor_across_screens
                                    } else {
                                        false
                                    }
                                };
                                if !enable_control {
                                    return;
                                }

                                unsafe { listener_setBlock(1); }
                                set_pos_in_remote_screen([state.remote_screen_size[0], ev[2] as f32]);
                                set_block(true);
                            }
                        }
                        ControlSide::TOP => {
                            if ev[2] <= state.self_screen_size[2] {
                                let enable_control = {
                                    if let Ok(s) = STATE.lock() {
                                        s.get_setting().enable_control && s.get_setting().cursor_across_screens
                                    } else {
                                        false
                                    }
                                };
                                if !enable_control {
                                    return;
                                }

                                unsafe { listener_setBlock(1); }
                                set_pos_in_remote_screen([ev[1] as f32, state.remote_screen_size[3]]);
                                set_block(true);
                            }
                        }
                        _ => {}
                    }
                }
            }
            MOUSE_DOWN => {
                set_mouse_button_hold(true);
            }
            MOUSE_UP => {
                set_mouse_button_hold(false);
            }
            _ => {}
        }
    }
}

extern "C" fn keyboard_handler(ev: *const InputType) {
    let ev = unsafe { slice::from_raw_parts(ev, 7) };
    let ev = &[
        ev[0] as i32,
        ev[1] as i32,
        ev[2] as i32,
        ev[3] as i32,
        ev[4] as i32,
        ev[5] as i32,
        ev[6] as i32,
    ];

    if !get_is_remote_alive() {
        return;
    }
    if get_block() {
        send_to_remote(ev);
    }
}

extern "C" fn hotkey_handler(hotkeys: *const [InputType; 7]) {
    info!("unblock hotkey triggered");
    thread::sleep(Duration::from_millis(100));

    if get_block() {
        set_block(false);
        unsafe { listener_setBlock(0); }
    } else {
        // Check if remote peer exists and control is enabled
        let can_control = {
            if let Ok(s) = STATE.lock() {
                s.remote_peer.is_some() && s.get_setting().enable_control
            } else {
                false
            }
        };
        if !can_control {
            return;
        }

        set_block(true);
        unsafe { listener_setBlock(1); }
        
        if let Ok(state) = get_listener_state().lock() {
            let pos = [
                (state.remote_screen_size[1] - state.remote_screen_size[0]) / 2.0,
                (state.remote_screen_size[3] - state.remote_screen_size[2]) / 2.0
            ];
            set_pos_in_remote_screen(pos);

            // Convert relative mouse movement to absolute
            let x = pos[0] as i32;
            let y = pos[1] as i32;
            let bytes_to_send = [1, x, y];
            send_to_remote(bytes_to_send.as_slice());
        }
    }

    // Notify remote side to release keys
    let hotkeys = unsafe { slice::from_raw_parts(hotkeys, 2) };
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

pub fn init() {
    thread::spawn(|| unsafe {
        listener_init(mouse_handler, keyboard_handler, hotkey_handler);
        listener_listen();
    });
}

pub fn release() {
    if get_is_remote_alive() {
        set_block(false);
        set_is_remote_alive(false);
        
        if let Ok(state) = get_listener_state().lock() {
            set_pos_in_remote_screen([state.remote_screen_size[0], state.pos_in_remote_screen[1]]);
        }
        
        unsafe { listener_setBlock(0); }
        #[cfg(target_os = "macos")]
        unsafe { power_set_replay_prevent(0); }
    }
}

pub fn keepalive() {
    set_is_remote_alive(true);
}