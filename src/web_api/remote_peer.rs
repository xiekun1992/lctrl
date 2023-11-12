use crate::global::device::RemoteDevice;
use crate::global::state::STATE;
use crate::input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE};
use actix_web::{delete, get, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RemoteSetting {
    pub ip: String,
    pub side: ControlSide,
}

#[put("/remote_peer")]
pub async fn put(setting: web::Query<RemoteSetting>) -> impl Responder {
    let mut state = STATE.lock().unwrap();
    let remote = state.find_remote_by_ip(&setting.ip.as_str());
    if let Some(rdev) = remote.clone() {
        unsafe {
            REMOTE_SCREEN_SIZE = rdev.screen_size.clone();
            SELF_SCREEN_SIZE = state.screen_size.clone();
            SIDE = setting.side;
            state.set_remote_peer(remote);
        }
    }
    HttpResponse::Ok().json(())
}

#[delete("/remote_peer")]
pub async fn delete() -> impl Responder {
    let mut state = STATE.lock().unwrap();
    state.set_remote_peer(None);
    unsafe {
        SIDE = ControlSide::NONE;
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
    unsafe {
        match STATE.lock().unwrap().remote_peer.as_ref() {
            Some(p) => HttpResponse::Ok().json(RemotePeer {
                remote: p.clone(),
                side: SIDE.clone(),
            }),
            None => HttpResponse::Ok().json(()),
        }
    }
}
