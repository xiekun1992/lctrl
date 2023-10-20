use actix_web::{get, HttpResponse, Responder};

use crate::global::state::STATE;

#[get("/remotes")]
pub async fn get() -> impl Responder {
    let remotes = STATE.lock().unwrap().get_remote();
    // web::Json(res)
    HttpResponse::Ok().json(remotes)
}
