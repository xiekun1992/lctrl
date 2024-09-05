use std::env;

// use chrono::Local;
use env_logger::Builder;
use global::{db, state::RECT};
use log::{error, info};

mod discover;
mod global;
mod input;
mod system_service;
mod web_api;

#[link(name = "libcapture")]
extern "C" {
    fn get_screen_size() -> RECT;
}

fn main() -> Result<(), i32> {
    Builder::new().filter(None, log::LevelFilter::Debug).init();

    info!("log module init");
    let args: Vec<String> = env::args().collect();

    #[cfg(target_os = "windows")]
    if let Some(_) = args.iter().find(|arg| (**arg).eq("--run_as_service")) {
        system_service::init();
    } else if let Some(_) = args.iter().find(|arg| (**arg).eq("--run_as_app")) {
        global::init();
        discover::init();
        input::init();
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
    return Err(0);
}
