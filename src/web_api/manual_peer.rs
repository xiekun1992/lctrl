use actix_web::{put, web, HttpResponse, Responder};
use tracing::info;

use crate::global::{device::RemoteDevice, state::STATE};

use super::dto::Params;

#[put("/manual_peer")]
pub async fn put(params: web::Query<Params>) -> impl Responder {
    info!("addr: {:?}", params);
    match reqwest::get(format!(
        "http://{}:18000/api/device/remote?addr={}",
        params.addr, params.addr
    ))
    .await
    {
        Ok(res) => match res.text().await {
            Ok(txt) => {
                if txt.len() > 0 {
                    let remote_device = RemoteDevice::from_json(txt);
                    match STATE.try_lock() {
                        Ok(mut state) => {
                            state.add_manual_remote(remote_device);
                            HttpResponse::Ok().json(())
                        }
                        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
                    }
                } else {
                    HttpResponse::NotFound().json(())
                }
            }
            Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        },
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
