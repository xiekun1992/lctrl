mod device;
mod file;
mod remote_peer;
mod remotes;
mod ws;
use std::{fs, vec};

use actix_cors::Cors;
use actix_files::Files;
use actix_multipart::form::{tempfile::TempFileConfig, MultipartFormConfig};
use actix_web::{http, middleware, App, HttpServer};

#[actix_web::main]
pub async fn web_main() -> std::io::Result<()> {
    fs::create_dir_all("./tmp").expect("tmp dir init failed");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    // .allowed_headers(vec![
                    //     http::header::AUTHORIZATION,
                    //     http::header::ACCEPT,
                    //     http::header::CONTENT_TYPE,
                    // ])
                    .allow_any_header()
                    .allow_any_method()
                    .expose_any_header()
                    .max_age(3600),
            )
            .app_data(
                MultipartFormConfig::default()
                    .memory_limit(usize::MAX)
                    .total_limit(usize::MAX),
            )
            .app_data(TempFileConfig::default().directory("./tmp"))
            // .service(Files::new("/static", "./static/build/static"))
            // .service(Files::new("/static", "./static").show_files_listing())
            .service(ws::index)
            .service(file::post)
            .service(device::get)
            .service(remotes::get)
            .service(remotes::post)
            .service(remotes::delete)
            .service(remote_peer::get)
            .service(remote_peer::put)
            .service(remote_peer::delete)
            .service(Files::new("/", "./static/build/"))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
