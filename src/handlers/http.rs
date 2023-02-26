use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    config::server_config::{ServerConfig, ServerLocationConfig},
    worker::Worker,
};

use super::{http_proxy::handle_proxy, http_static::handle_static};

pub async fn handle_location(req: HttpRequest, location: &ServerLocationConfig) -> HttpResponse {
    let root = &location.root;
    let proxy_to = &location.proxy_to;

    if root.is_some() {
        return handle_static(req, location).await;
    } else if proxy_to.is_some() {
        return handle_proxy(req, location).await;
    }

    HttpResponse::NotFound().body(format!("Error: No handle for location: {}", req.path()))
}

pub async fn handle_server(req: HttpRequest, server: &ServerConfig) -> HttpResponse {
    let path = req.path();

    for location in server.locations.as_ref().unwrap() {
        if path.starts_with(&location.path) {
            return handle_location(req, location).await;
        }
    }

    HttpResponse::NotFound().body(format!("Error: Location Not Found for: {}", req.path()))
}

pub async fn handle_request(
    req: HttpRequest,
    arc_worker: web::Data<Mutex<Worker>>,
) -> HttpResponse {
    // Host getter.
    let host_header = req.headers().get("host");
    let host: &str;

    if host_header.is_some() {
        host = host_header.unwrap().to_str().unwrap();
    } else {
        host = "";
    }

    // Server getter.
    let mut worker = arc_worker.lock().unwrap();
    let mut server = worker.get_site(host.to_string());

    if server.is_none() {
        server = worker.get_default_site();
    }

    if server.is_some() {
        handle_server(req, server.unwrap()).await
    } else {
        HttpResponse::NotFound().body(format!(
            "Error: Server Not Found, Host: {}, Path: {}",
            host,
            req.path(),
        ))
    }
}
