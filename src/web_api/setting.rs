use actix_web::{get, put, web};
use actix_web::{HttpResponse, Responder};
use serde::Deserialize;

use crate::global::state::STATE;
use crate::web_api::dto::ScreenSetting;

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

#[put("/setting/screens")]
async fn set_screens(body: web::Json<ScreenSetting>) -> impl Responder {
    match STATE.try_lock() {
        Ok(mut state) => {
            state.set_screens_setting(body.0);
            HttpResponse::Ok().json(())
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/setting")]
async fn get_setting() -> impl Responder {
    match STATE.try_lock() {
        Ok(state) => {
            let setting = state.get_setting();
            HttpResponse::Ok().json(setting)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
