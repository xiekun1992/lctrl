use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, HttpResponse, Responder};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/file")]
pub async fn post(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    println!("file upload: {}", form.files.len());
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        println!("saving to {path}");
        f.file.persist(path).unwrap();
    }
    HttpResponse::Ok().json(true)
}

// #[get("/file")]
// pub async fn get() -> impl Responder {}
