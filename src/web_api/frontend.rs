use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use include_dir::include_dir;
use include_dir::Dir;

static PROJECT_DIR: Dir = include_dir!("./static/build/");

#[get("/")]
pub async fn get() -> impl Responder {
    let index_page = PROJECT_DIR.get_file("index.html").unwrap().contents();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(index_page)
}

#[get("/{path:.*}")]
pub async fn get_resource(path: web::Path<String>) -> impl Responder {
    let p = path.into_inner();
    println!("request {}", p);
    if p == "" {
        let index_page = PROJECT_DIR.get_file("index.html").unwrap().contents();
        HttpResponse::Ok()
            .content_type("text/html")
            .body(index_page)
    } else {
        let resource = PROJECT_DIR.get_file(p).unwrap().contents();
        HttpResponse::Ok().body(resource)
    }

    // let resource = PROJECT_DIR.get_file(p).unwrap().contents();
    // HttpResponse::Ok().body("")
}

// #[get("{path}")]
// pub async fn get_resource(path: web::Path<String>) -> impl Responder {
//     let p = path.into_inner();
//     println!("request {}", p.clone());

//     let resource = PROJECT_DIR.get_file(p).unwrap().contents();
//     HttpResponse::Ok().body(resource)
// }
