use actix_web::{web, HttpRequest, HttpResponse};

use crate::worker::Worker;

pub async fn handler(req: HttpRequest, worker: web::Data<Worker>) -> HttpResponse {
    let host_header = req.headers().get("host");
    let host: &str;

    if host_header.is_some() {
        host = host_header.unwrap().to_str().unwrap();
    } else {
        host = "";
    }

    HttpResponse::Ok().body(format!(
        "Host: {}, Path: {}, Servers: {}",
        host,
        req.path(),
        worker.sites.len()
    ))
}
