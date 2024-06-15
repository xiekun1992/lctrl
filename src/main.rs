use std::env;

// use chrono::Local;
use env_logger::Builder;
use global::{
    db,
    state::{Rect, RECT},
};
use log::info;
use std::fs::File;
// use std::io::Write;

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
    // File::create("output.log").unwrap();
    // let log_file = Box::new(File::open("output.log").unwrap());
    Builder::new()
        // .target(env_logger::Target::Pipe(log_file))
        .filter(None, log::LevelFilter::Debug)
        .init();

    info!("log module init");
    let args: Vec<String> = env::args().collect();
    if cfg!(target_os = "windows") {
        // if let Some(_) = args.iter().find(|arg| (**arg).eq("--run_as_service")) {
        //     system_service::init();
        // } else if let Some(_) = args.iter().find(|arg| (**arg).eq("--run_as_app")) {
        //     global::init();
        //     discover::init();
        //     input::init();
        //     let _ = web_api::web_main();
        //     system_service::stop();
        // } else {
        //     system_service::bootstrap();
        // }
    } else {
        if let Some(_) = args.iter().find(|arg| (**arg).eq("--get-screen-size")) {
            let rect = unsafe { get_screen_size() };
            println!("{:?}", rect);
            let conn = db::DB::new();
            // println!("{:?}", conn.get_current_device());
            conn.set_current_device(&rect);
        } else {
            global::init();
            discover::init();
            input::init();
            let _ = web_api::web_main();
        }
    }
    return Err(0);
}
