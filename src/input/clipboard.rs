use std::{
    ffi::{c_char, c_int, c_long, c_uchar, c_ushort, CStr, OsStr, OsString},
    iter::once,
    os::windows::prelude::{OsStrExt, OsStringExt},
    slice, thread,
    time::Duration,
};

type CClipboardHandler = extern "C" fn();
#[link(name = "libcapture")]
extern "C" {
    fn clipboard_init();
    fn clipboard_dispose();
    fn read_text() -> *const c_ushort;
    fn capture(cb: CClipboardHandler);
    fn write_text(text: *const c_ushort) -> c_int;

    // fn utils_open(path: *const c_ushort);
}

pub fn init() {
    extern "C" fn cb() {
        unsafe {
            let text = read_text();
            let text1 = const_u16_to_string(text);
            println!("clipboard updated: {}", text1);
        }
    }
    unsafe {
        clipboard_init();

        let content = String::from("// 计算以 null 结尾的 u16 数组的长度");
        let ptr = string_to_vec_u16(content);
        println!("{:?}", ptr);
        write_text(ptr.as_ptr());
        // capture(cb);
        clipboard_dispose();
    }
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
