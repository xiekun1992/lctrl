use actix_web::{put, web};
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

use crate::global::state::STATE;

#[derive(Deserialize)]
struct Param {
    active: bool,
}

#[put("/setting/auto_discover")]
async fn set_auto_discover(query: web::Query<Param>) -> impl Responder {
    match STATE.try_lock() {
        Ok(mut state) => {
            state.set_auto_discover(query.active);
            HttpResponse::Ok().json(())
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
