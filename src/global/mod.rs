use self::{device::get_interfaces, state::State};
use crate::input::listener::{ControlSide, ListenerState};
use lazy_static::lazy_static;
use std::{sync::{Mutex, OnceLock}, thread, time::Duration};

pub mod db;
pub mod device;
pub mod setting;
pub mod state;

// ============================================================================
// Global State Definitions - All global variables are centralized here
// ============================================================================

// Application state - shared across the application
lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

/// Listener state - for input event handling
static LISTENER_STATE: OnceLock<Mutex<ListenerState>> = OnceLock::new();

/// Get the listener state reference
pub fn get_listener_state() -> &'static Mutex<ListenerState> {
    LISTENER_STATE.get_or_init(|| Mutex::new(ListenerState::new()))
}

// ============================================================================
// Listener State Access API
// ============================================================================

/// Set remote screen size
pub fn set_remote_screen_size(size: [f32; 4]) {
    if let Ok(mut state) = get_listener_state().lock() {
        state.remote_screen_size = size;
    }
}

/// Get remote screen size
pub fn get_remote_screen_size() -> [f32; 4] {
    if let Ok(state) = get_listener_state().lock() {
        state.remote_screen_size
    } else {
        [0.0, 0.0, 0.0, 0.0]
    }
}

/// Set self screen size
pub fn set_self_screen_size(size: [i32; 4]) {
    if let Ok(mut state) = get_listener_state().lock() {
        state.self_screen_size = size;
    }
}

/// Get self screen size
pub fn get_self_screen_size() -> [i32; 4] {
    if let Ok(state) = get_listener_state().lock() {
        state.self_screen_size
    } else {
        [0, 0, 0, 0]
    }
}

/// Set control side
pub fn set_side(side: ControlSide) {
    if let Ok(mut state) = get_listener_state().lock() {
        state.side = side;
    }
}

/// Get control side
pub fn get_side() -> ControlSide {
    if let Ok(state) = get_listener_state().lock() {
        state.side
    } else {
        ControlSide::NONE
    }
}

/// Set block status
pub fn set_block(block: bool) {
    if let Ok(mut state) = get_listener_state().lock() {
        state.block = block;
    }
}

/// Get block status
pub fn get_block() -> bool {
    if let Ok(state) = get_listener_state().lock() {
        state.block
    } else {
        false
    }
}

/// Set is_remote_alive status
pub fn set_is_remote_alive(alive: bool) {
    if let Ok(mut state) = get_listener_state().lock() {
        state.is_remote_alive = alive;
    }
}

/// Get is_remote_alive status
pub fn get_is_remote_alive() -> bool {
    if let Ok(state) = get_listener_state().lock() {
        state.is_remote_alive
    } else {
        false
    }
}

/// Get pos_in_remote_screen
pub fn get_pos_in_remote_screen() -> [f32; 2] {
    if let Ok(state) = get_listener_state().lock() {
        state.pos_in_remote_screen
    } else {
        [0.0, 0.0]
    }
}

/// Set pos_in_remote_screen
pub fn set_pos_in_remote_screen(pos: [f32; 2]) {
    if let Ok(mut state) = get_listener_state().lock() {
        state.pos_in_remote_screen = pos;
    }
}

/// Get mouse_button_hold
pub fn get_mouse_button_hold() -> bool {
    if let Ok(state) = get_listener_state().lock() {
        state.mouse_button_hold
    } else {
        false
    }
}

/// Set mouse_button_hold
pub fn set_mouse_button_hold(hold: bool) {
    if let Ok(mut state) = get_listener_state().lock() {
        state.mouse_button_hold = hold;
    }
}

// ============================================================================
// Initialization
// ============================================================================

pub fn init() {
    thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(1));

        let ifs = get_interfaces();
        {
            match STATE.lock() {
                Ok(mut state) => {
                    if state.cur_device.ifs.len() != ifs.len() {
                        state.cur_device.ifs = ifs;
                    } else if state.cur_device.ifs != ifs {
                        state.cur_device.ifs = ifs;
                    }
                }
                Err(_err) => {}
            }
        }
    });
}