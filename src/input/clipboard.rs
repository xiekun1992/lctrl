use std::{
    ffi::{c_int, c_ushort, OsStr},
    iter::once,
    os::windows::prelude::OsStrExt,
    slice, thread,
    time::Duration,
};

use tracing::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::global::state::STATE;

type CClipboardHandler = extern "C" fn();
// #[link(name = "libcapture")]
extern "C" {
    fn clipboard_init();
    fn clipboard_dispose();
    fn read_text() -> *const c_ushort;
    fn capture(cb: CClipboardHandler);
    fn write_text(text: *const c_ushort) -> c_int;

    // fn utils_open(path: *const c_ushort);
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClipboardBody {
    pub text: String,
}

extern "C" fn cb() {
    if let Ok(state) = STATE.try_lock() {
        if let Some(peer) = state.remote_peer.clone() {
            let url = format!("http://{}:18000/api/clipboard", peer.ip);
            unsafe {
                let text = read_text();
                let text1 = const_u16_to_string(text);
                debug!("clipboard updated: {}", text1);
                // send to remote peer
                let body = ClipboardBody { text: text1 };
                let body_str = match serde_json::to_string(&body) {
                    Ok(str) => str,
                    Err(err) => {
                        error!("{:?}", err);
                        "".to_string()
                    }
                };

                let client = reqwest::blocking::Client::new();
                match client
                    .put(url)
                    .header("content-type", "application/json")
                    .body(body_str)
                    .timeout(Duration::from_millis(500))
                    .send()
                {
                    Ok(res) => {
                        info!("{:?}", res);
                    }
                    Err(err) => {
                        error!("{:?}", err);
                    }
                };
            }
        }
    }
}

pub fn write_text_to_clipboard(content: String) -> () {
    let ptr = string_to_vec_u16(content);
    unsafe {
        // println!("{:?}", ptr);
        write_text(ptr.as_ptr());
        debug!("text wrote");
    }
    // let content = String::from("// 计算以 null 结尾的 u16 数组的长度");
    // let ptr = string_to_vec_u16(content);
    // println!("{:?}", ptr);
    // write_text(ptr.as_ptr());
}

pub fn init() {
    match thread::Builder::new()
        .name("clipboard thread".to_string())
        .spawn(|| unsafe {
            clipboard_init();
            capture(cb);
            clipboard_dispose();
        }) {
        _ => {}
    };
}

fn string_to_vec_u16(msg: String) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(msg.as_str())
        .encode_wide()
        .chain(once(0))
        .collect();
    return wide;
}

fn const_u16_to_string(wide_str_ptr: *const u16) -> String {
    // 计算以 null 结尾的 u16 数组的长度
    let mut len = 0;
    while unsafe { *wide_str_ptr.offset(len as isize) } != 0 {
        len += 1;
    }
    let wide_str = unsafe { slice::from_raw_parts(wide_str_ptr, len) };
    // println!("const_u16_to_string={:?}", wide_str);
    let str = String::from_utf16_lossy(wide_str);
    return str;
}
