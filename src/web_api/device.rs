use actix_web::{get, HttpResponse, Responder};

use crate::global::state::STATE;

#[get("/device")]
pub async fn get(req_body: String) -> impl Responder {
    println!("{}", req_body);
    let res = &STATE.lock().unwrap().cur_device; //.clone();
                                                 // HttpResponse::Ok().body(res)
                                                 // web::Json(res)
    HttpResponse::Ok().json(res)
}
