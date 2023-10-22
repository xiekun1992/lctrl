use actix_web::{get, HttpResponse, Responder};

use crate::global::state::STATE;

#[get("/device")]
pub async fn get() -> impl Responder {
    let res = &STATE.lock().unwrap().cur_device;
    HttpResponse::Ok().json(res)
}
