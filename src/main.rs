// use std::net::UdpSocket;
use actix_web::{HttpServer, App, get, Responder, HttpResponse, post, web};
use std::{thread::{self}, sync::{Arc, Mutex}};

mod global;
mod discover;

async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}
// #[get("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     println!("{}", req_body);
//     HttpResponse::Ok().body(req_body)
// }

// #[post("/echo")]
// async fn echo1(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

#[actix_web::main]
async fn web_main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/echo", web::get().to(echo))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await

}


fn main() {
    let state = Arc::new(global::state::State::new());
    discover::init(state);
    let _ = web_main();
}
