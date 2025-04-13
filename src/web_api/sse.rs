use std::time::Duration;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use tokio::time::interval;

use crate::{global::state::STATE, web_api::dto::RemoteDevices};

#[get("/sse")]
async fn sse_handler(_req: HttpRequest) -> impl Responder {
    let mut interval = interval(Duration::from_secs(1));
    let mut counter = 0;

    let event_stream = async_stream::stream! {
        loop {
            interval.tick().await;
            match STATE.try_lock() {
                Ok(state) => {
                    let remotes = RemoteDevices {
                        remotes: state.get_remote(),
                        manual_remotes: state.get_manual_remote(),
                    };
                    counter+=1;
                    let payload = format!("id: {counter}\nevent: update.remotes\ndata: {}\n\n", remotes.to_json());
                    yield Ok::<_, actix_web::Error>(web::Bytes::from(payload));
                }
                Err(_) => {}
            }
        }
    };

    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(event_stream)
}
