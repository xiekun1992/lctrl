use actix_web::{HttpServer, App};

mod web_api;
mod global;
mod discover;
mod input;

#[actix_web::main]
async fn web_main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
    discover::init();
    input::init();
    let _ = web_main();
}
