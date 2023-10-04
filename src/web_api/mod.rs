pub mod device {
    use actix_web::{get, HttpResponse, Responder, web};

    use crate::global::{self, state::STATE};

    #[get("/device")]
    pub async fn get(req_body: String) -> impl Responder {
        println!("{}", req_body);
        let res = &STATE.lock().unwrap().cur_device;//.clone();
        // HttpResponse::Ok().body(res)
        // web::Json(res)
        HttpResponse::Ok().json(res)
    }
}

pub mod remotes {
    use actix_web::{get, HttpResponse, Responder, web, put, delete};

    use crate::global::{self, state::{State, STATE}};

    #[get("/remotes")]
    pub async fn get() -> impl Responder {
        let remotes = STATE.lock().unwrap().get_remote();
        // web::Json(res)
        HttpResponse::Ok().json(remotes)
    }

    #[put("/remote_peer/{ip}")]
    pub async fn put(ip: web::Path<String>) -> impl Responder {
        let mut state = STATE.lock().unwrap();
        let remote = state.find_remote_by_ip(&ip.as_str());
        state.set_remote_peer(remote);
        HttpResponse::Ok().json(())
    }

    #[delete("/remote_peer")]
    pub async fn delete() -> impl Responder {
        let mut state = STATE.lock().unwrap();
        state.set_remote_peer(None);
        HttpResponse::Ok().json(())
    }
}
