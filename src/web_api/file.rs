use std::{fs, io::Read};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, HttpResponse, Responder};
use log::info;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/file")]
pub async fn post(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    info!("file upload: {}", form.files.len());
    if fs::metadata("./tmp").is_err() {
        fs::create_dir("./tmp").unwrap();
    }
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        info!("saving to {path}, {:?}", f.file.path());
        fs::copy(f.file.path(), path).unwrap();
        // f.file.persist(path).unwrap();
    }
    HttpResponse::Ok().json(true)
}

// #[get("/file")]
// pub async fn get() -> impl Responder {}
