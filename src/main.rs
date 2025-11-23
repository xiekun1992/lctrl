use std::env;

// use chrono::Local;
// use env_logger::Builder;
use global::{db, state::RECT};
use tracing::Level;
use tracing::{error, info};

mod discover;
mod global;
mod input;
mod system_service;
mod web_api;

// #[link(name = "libcapture")]
extern "C" {
    #[cfg(target_os = "linux")]
    fn get_screen_size() -> RECT;
}

fn main() -> Result<(), i32> {
    let file_appender = tracing_appender::rolling::daily("./", "lctrl");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // 将日志文件添加到 tracing_subscriber 中

    tracing_subscriber::fmt()
        .with_target(true)
        // .with_ansi(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        // .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .init();

    info!("log module init");
    let args: Vec<String> = env::args().collect();

    #[cfg(target_os = "windows")]
    if let Some(_) = args.iter().find(|arg| (**arg).eq("--run_as_service")) {
        system_service::init();
    } else if let Some(_) = args.iter().find(|arg| (**arg).eq("--run_as_app")) {
        global::init();
        discover::init();
        input::init();
        system_service::listen_service_close();
        let _ = web_api::web_main();
        system_service::stop();
    } else {
        system_service::bootstrap();
    }

    #[cfg(target_os = "linux")]
    if let Some(_) = args.iter().find(|arg| (**arg).eq("--get-screen-size")) {
        let rect = unsafe { get_screen_size() };
        info!("{:?}", rect);
        let conn = db::DB::new();
        conn.set_current_device(&rect);
        // println!("{:?}", conn.get_current_device());
    } else if let Some(_) = args.iter().find(|arg| (**arg).eq("--run_as_app")) {
        global::init();
        discover::init();
        input::init();
        let _ = web_api::web_main();
    } else {
        error!("please run ./run.sh in current directory");
    }

    #[cfg(target_os = "macos")]
    {
        global::init();
        discover::init();
        input::init();
        let _ = web_api::web_main();
    }
    return Err(0);
}
