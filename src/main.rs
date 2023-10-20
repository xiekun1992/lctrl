use std::fs;

use actix_files::Files;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{middleware, App, HttpServer};

mod discover;
mod global;
mod input;
mod web_api;

#[actix_web::main]
async fn web_main() -> std::io::Result<()> {
    fs::create_dir_all("./tmp").expect("tmp dir init failed");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(Files::new("/static", "./static").show_files_listing())
            .service(web_api::file::post)
            .service(web_api::device::get)
            .service(web_api::remotes::get)
            .service(web_api::remote_peer::get)
            .service(web_api::remote_peer::put)
            .service(web_api::remote_peer::delete)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

fn main() {
    // discover::init();
    // input::init();
    let _ = web_main();
}
