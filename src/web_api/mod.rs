mod clipboard;
mod device;
mod dto;
mod file;
mod frontend;
mod launch;
mod manual_peer;
mod remote_peer;
mod remotes;
mod setting;
mod sse;
// use std::{fs, vec};

use actix_cors::Cors;
// use actix_files::Files;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web;
use actix_web::{middleware, App, HttpServer};
use tracing::info;

#[actix_web::main]
pub async fn web_main() -> std::io::Result<()> {
    // fs::create_dir_all("./tmp").expect("tmp dir init failed");

    info!("web module init");
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
            // .app_data(TempFileConfig::default().directory("./tmp"))
            // .service(Files::new("/static", "./static/build/static"))
            // .service(Files::new("/static", "./static").show_files_listing())
            .service(
                web::scope("/api")
                    .service(sse::sse_handler)
                    .service(launch::put)
                    // .service(clipboard::put)
                    .service(file::post)
                    .service(device::get)
                    .service(device::get_as_remote)
                    .service(remotes::get)
                    .service(remotes::post)
                    .service(remotes::delete)
                    .service(remote_peer::get)
                    .service(remote_peer::put)
                    .service(remote_peer::delete)
                    .service(manual_peer::put)
                    .service(setting::set_auto_discover),
            )
            // .service(frontend::get)
            .service(frontend::get_resource)
        // .default_service(|| HttpResponse::Ok().status(404).body("Not Found"))
    })
    .bind(("0.0.0.0", 18000))?
    .run()
    .await
}
