use actix_multipart::form::json;
use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    global::{device::RemoteDevice, state::STATE},
    web_api::dto::Params,
};

#[get("/device")]
pub async fn get() -> impl Responder {
    let res = &STATE.lock().unwrap().cur_device;
    HttpResponse::Ok().json(res)
}

#[get("/device/remote")]
pub async fn get_as_remote(params: web::Query<Params>) -> impl Responder {
    match STATE.try_lock() {
        Ok(state) => {
            let res = state
                .cur_device
                .ifs
                .iter()
                .find(|el| el.addr.to_string().eq(&params.addr));

            match res {
                Some(iface) => {
                    let dev = RemoteDevice {
                        hostname: state.cur_device.hostname.clone(),
                        ip: iface.addr.to_string(),
                        netmask: iface.netmask.to_string(),
                        mac_addr: iface.mac_addr.to_string(),
                        screen_size: state.screen_size.clone(),
                        alive_timestamp: 0,
                    };
                    HttpResponse::Ok().json(dev)
                }
                None => HttpResponse::NotFound().json(()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
