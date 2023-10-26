use actix_web::{
    delete, get, post,
    web::{self, Json, Query},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::global::{device::RemoteDevice, state::STATE};

#[get("/remotes")]
pub async fn get() -> impl Responder {
    let remotes = STATE.lock().unwrap().get_remote();
    // web::Json(res)
    HttpResponse::Ok().json(remotes)
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
