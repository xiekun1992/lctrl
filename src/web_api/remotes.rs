use crate::global::{device::RemoteDevice, state::STATE};
use crate::web_api::dto::RemoteDevices;
use actix_web::{
    delete, get, post,
    web::{Json, Query},
    HttpResponse, Responder,
};
use serde::Deserialize;

#[get("/remotes")]
pub async fn get() -> impl Responder {
    match STATE.try_lock() {
        Ok(state) => HttpResponse::Ok().json(RemoteDevices {
            remotes: state.get_remote(),
            manual_remotes: state.get_manual_remote(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
    // web::Json(res)
}

#[post("/remotes")]
pub async fn post(remote: Json<RemoteDevice>) -> impl Responder {
    println!("{:?}", remote);
    {
        STATE.lock().unwrap().add_remote(remote.0);
    }
    HttpResponse::Ok().json(())
}
#[derive(Deserialize)]
pub struct RemoteQuery {
    ip: String,
}
#[delete("/remotes")]
pub async fn delete(query: Query<RemoteQuery>) -> impl Responder {
    {
        STATE.lock().unwrap().del_remote(query.ip.clone());
    }
    HttpResponse::Ok().json(())
}
