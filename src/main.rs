// use chrono::Local;
use env_logger::Builder;
use log::info;
// use std::fs::File;
// use std::io::Write;

mod discover;
mod global;
mod input;
mod web_api;

fn main() {
    // let log_file = Box::new(File::open("output.log").unwrap());
    Builder::new()
        .filter(None, log::LevelFilter::Debug)
        // .target(env_logger::Target::Pipe(log_file))
        .init();

    info!("log module init");

    global::init();

    discover::init();

    input::init();

    let _ = web_api::web_main();
}
