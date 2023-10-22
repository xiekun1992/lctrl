mod device;
mod file;
mod remote_peer;
mod remotes;

use std::fs;

use actix_files::Files;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
pub async fn web_main() -> std::io::Result<()> {
    fs::create_dir_all("./tmp").expect("tmp dir init failed");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(Files::new("/static", "./static").show_files_listing())
            .service(file::post)
            .service(device::get)
            .service(remotes::get)
            .service(remote_peer::get)
            .service(remote_peer::put)
            .service(remote_peer::delete)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
