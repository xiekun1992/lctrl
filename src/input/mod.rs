

use std::{ffi::{c_int, c_long, c_char, CStr, c_uchar, OsString, OsStr, c_ushort}, thread, time::Duration, slice, os::windows::prelude::{OsStrExt, OsStringExt}, iter::once};

#[repr(C)]
// #[no_mangle]
struct Square {
    a: c_int,
    b: c_int
}
#[repr(C)]
enum MouseButton {
    MouseLeft = 1,
    MouseMiddle,
    MouseRight
}
#[repr(C)]
enum MouseWheel {
    WheelUp = -1,
    WheelDown = 1
}
enum MouseType {
    MouseWheel = 0,
    MouseMove,
    MouseDown,
    MouseUp,
    KeyDown,
    KeyUp
}
type CInputHandler = extern fn(ev: *const c_long);
type CClipboardHandler = extern fn();
#[link(name = "libcapture")]
extern "C" {
    fn mouse_move(x: c_int, y: c_int);
    fn mouse_wheel(direction: MouseWheel);
    fn mouse_down(button: MouseButton);
    fn mouse_up(button: MouseButton);

    fn listener_init(
        mouseHanlder: CInputHandler,
        keyboardHanlder: CInputHandler
    );
    fn listener_listen();
    fn charToKeycode(scancode: c_int) -> c_int;

    fn clipboard_init();
    fn clipboard_dispose();
    fn read_text() -> *const c_ushort;
    fn capture(cb: CClipboardHandler);
    fn write_text(text: *const c_ushort) -> c_int;

    fn utils_open(path: *const c_ushort);
}


fn string_to_vec_u16(msg: String) -> Vec<u16> {
  let wide: Vec<u16> = OsStr::new(msg.as_str()).encode_wide().chain(once(0)).collect();
  return wide;
}

fn const_u16_to_string(wide_str_ptr: *const u16) -> String {
  // 计算以 null 结尾的 u16 数组的长度
  let mut len = 0;
  while unsafe { *wide_str_ptr.offset(len as isize) } != 0 {
      len += 1;
  }
  let wide_str = unsafe {
      slice::from_raw_parts(wide_str_ptr, len)
  };
  // println!("const_u16_to_string={:?}", wide_str);
  let str = String::from_utf16_lossy(wide_str);
  return str;
}
