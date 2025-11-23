use std::env;

// use chrono::Local;
// use env_logger::Builder;
use global::{db, state::Rect, state::RECT};
// use tracing::Level;
use tracing::{error, info};

mod discover;
mod global;
mod input;
mod system_service;
mod web_api;

#[cfg(target_os = "linux")]
extern "C" {
    fn get_screen_size() -> RECT;
    fn get_screens(count: *mut i32) -> *const Rect;
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
        info!("screens area {:?}", rect);

        let screens = unsafe {
            let mut count = 0;
            let screens_rects = get_screens(&mut count);
            if screens_rects.is_null() || count == 0 {
                vec![]
            } else {
                std::slice::from_raw_parts(screens_rects, count as usize).to_vec()
            }
        };
        info!("screens {:?}", screens);

        let conn = db::DB::new();
        conn.set_current_device(&rect);
        conn.set_screens(&screens);
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
