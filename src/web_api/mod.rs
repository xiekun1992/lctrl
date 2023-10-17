pub mod device {
    use actix_web::{get, web, HttpResponse, Responder};

    use crate::global::state::STATE;

    #[get("/device")]
    pub async fn get(req_body: String) -> impl Responder {
        println!("{}", req_body);
        let res = &STATE.lock().unwrap().cur_device; //.clone();
                                                     // HttpResponse::Ok().body(res)
                                                     // web::Json(res)
        HttpResponse::Ok().json(res)
    }
}

pub mod remotes {
    use actix_web::{delete, get, put, web, HttpResponse, Responder};

    use crate::global::state::STATE;

    #[get("/remotes")]
    pub async fn get() -> impl Responder {
        let remotes = STATE.lock().unwrap().get_remote();
        // web::Json(res)
        HttpResponse::Ok().json(remotes)
    }
}

pub mod remote_peer {
    use crate::global::state::STATE;
    use crate::input::listener::{ControlSide, REMOTE_SCREEN_SIZE, SELF_SCREEN_SIZE, SIDE};
    use actix_web::{delete, get, put, web, HttpResponse, Responder};

    #[put("/remote_peer/{ip}")]
    pub async fn put(ip: web::Path<String>) -> impl Responder {
        let mut state = STATE.lock().unwrap();
        let remote = state.find_remote_by_ip(&ip.as_str());
        if let Some(rdev) = remote.clone() {
            unsafe {
                REMOTE_SCREEN_SIZE = rdev.screen_size.clone();
                SELF_SCREEN_SIZE = state.screen_size.clone();
                SIDE = ControlSide::RIGHT;
                state.set_remote_peer(remote);
            }
        }
        HttpResponse::Ok().json(())
    }

    #[delete("/remote_peer")]
    pub async fn delete() -> impl Responder {
        let mut state = STATE.lock().unwrap();
        state.set_remote_peer(None);
        HttpResponse::Ok().json(())
    }

    #[get("/remote_peer")]
    pub async fn get() -> impl Responder {
        match STATE.lock().unwrap().remote_peer.as_ref() {
            Some(p) => HttpResponse::Ok().json(p),
            None => HttpResponse::Ok().json(()),
        }
    }
}
