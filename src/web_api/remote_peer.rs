use crate::global::device::RemoteDevice;
use crate::global::state::STATE;
use crate::input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE};
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
                unsafe {
                    REMOTE_SCREEN_SIZE = rdev.screen_size.clone().to_arr();
                    SELF_SCREEN_SIZE = state.screen_size.clone().to_arr();
                    SIDE = setting.side;
                    state.set_remote_peer(remote, &SIDE.clone());
                }
            }
            HttpResponse::Ok().json(())
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/remote_peer")]
pub async fn delete() -> impl Responder {
    let mut state = STATE.lock().unwrap();
    unsafe {
        SIDE = ControlSide::NONE;
        state.set_remote_peer(None, &SIDE);
    }
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
