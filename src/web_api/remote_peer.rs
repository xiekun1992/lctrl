use crate::global::device::RemoteDevice;
use crate::global::STATE;
use crate::input::listener::ControlSide;
use crate::global::{set_remote_screen_size, set_self_screen_size, set_side, get_side};
use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Deserialize)]
pub struct RemoteSetting {
    pub ip: String,
    pub side: ControlSide,
}

#[put("/remote_peer")]
pub async fn put(setting: web::Query<RemoteSetting>) -> impl Responder {
    match STATE.try_lock() {
        Ok(mut state) => {
            let remote = state.find_remote_by_ip(&setting.ip.as_str());
            if let Some(rdev) = remote.clone() {
                set_remote_screen_size(rdev.screen_size.clone().to_float_arr());
                set_self_screen_size(state.screen_size.clone().to_arr());
                set_side(setting.side);
                state.set_remote_peer(remote, &get_side());
            }
            HttpResponse::Ok().json(())
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/remote_peer")]
pub async fn delete() -> impl Responder {
    let mut state = STATE.lock().unwrap();
    set_side(ControlSide::NONE);
    state.set_remote_peer(None, &get_side());
    HttpResponse::Ok().json(())
}

#[derive(Deserialize, Serialize)]
struct RemotePeer {
    remote: RemoteDevice,
    side: ControlSide,
}

#[get("/remote_peer")]
pub async fn get() -> impl Responder {
    match STATE.lock() {
        Ok(state) => match state.get_remote_peer() {
            Some(p) => HttpResponse::Ok().json(RemotePeer {
                remote: p.clone(),
                side: state.side.clone(),
            }),
            None => HttpResponse::Ok().json(()),
        },
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::NotFound().json(())
        }
    }
}
