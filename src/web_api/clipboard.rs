// use actix_web::{put, web, HttpResponse, Responder};
// use tracing::info;

// use crate::input::clipboard::{write_text_to_clipboard, ClipboardBody};

// #[put("/clipboard")]
// pub async fn put(body: web::Json<ClipboardBody>) -> impl Responder {
//     info!("{:?}", body.text.clone());
//     write_text_to_clipboard(body.text.clone());
//     HttpResponse::Ok().json(true)
// }
