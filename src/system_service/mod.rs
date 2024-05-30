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

#[cfg(target_os = "windows")]
pub fn add_windows_firewall_rule() {
    if let Ok(app_path) = env::current_exe() {
        let app_name = app_path.file_name().unwrap().to_str().unwrap();
        let cmd = format!("netsh advfirewall firewall show rule name=\"{}\"", app_name);
        info!("{}", cmd);
        let output = Command::new("cmd")
            .args(["/C", cmd.as_str()])
            .output()
            .expect("Failed to show firewall rule");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            info!("Command executed successfully:");
            info!("{}", stdout);
            return;
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Command failed to execute:");
            error!("{}", stderr);
        }

        if let Some(app_path) = app_path.to_str() {
            let cmd = format!("netsh advfirewall firewall add rule name=\"{}\" dir=in action=allow program={} enable=yes", app_name, app_path);
            info!("{}", cmd);
            let output = Command::new("cmd")
                .args(["/C", cmd.as_str()])
                .output()
                .expect("Failed to add firewall rule");
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                info!("Command executed successfully:");
                info!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("Command failed to execute:");
                error!("{}", stderr);
            }
        }
    }
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
    // 检查服务是否存在，不存在则注册并启动服务，存在则启动服务并退出
    unsafe {
        if is_run_as_admin() == 0 {
            run_as_admin();
        } else {
            if cfg!(target_os = "windows") {
                add_windows_firewall_rule();
            }
            create_service();
            start_service();
        }
    };
}
pub fn stop() {
    // TODO: 停止服务
    unsafe {
        stop_service();
    }
}
