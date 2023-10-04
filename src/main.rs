use actix_web::{HttpServer, App};

mod web_api;
mod global;
mod discover;

#[actix_web::main]
async fn web_main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web_api::device::get)
            .service(web_api::remotes::get)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

fn main() {
    discover::init();
    let _ = web_main();
}
