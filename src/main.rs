mod discover;
mod global;
mod input;
mod web_api;

fn main() {
    discover::init();
    input::init();
    let _ = web_api::web_main();
}
