use std::{
    env,
    ffi::c_int,
    process::{exit, Command},
};

use log::{error, info};

#[link(name = "libcapture")]
extern "C" {
    fn register_service() -> c_int;
    fn create_service();
    fn start_service();
    fn stop_service();

    fn is_run_as_admin() -> c_int;
    fn run_as_admin();
}

pub fn init() {
    // 注册服务
    unsafe {
        if register_service() == 0 {
            info!("system service module init");
        } else {
            error!("system service module init failed");
        }
    }
}
pub fn bootstrap() {
    // TODO: 检查服务是否存在，不存在则注册并启动服务，存在则启动服务并退出
    unsafe {
        if is_run_as_admin() == 0 {
            run_as_admin();
        }
        create_service();
        start_service();
    };
}
pub fn stop() {
    // TODO: 停止服务
    unsafe {
        stop_service();
    }
}
