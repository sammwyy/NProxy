use axum::{extract::State, response::Response};
use hyper::{Body, Request, StatusCode};

use crate::{
    config::server_config::{ServerConfig, ServerLocationConfig},
    worker::Worker,
};

use super::{http_proxy::handle_proxy, http_static::handle_static};

pub async fn handle_location(
    req: Request<Body>,
    location: &ServerLocationConfig,
) -> Response<Body> {
    let root = &location.root;
    let proxy_to = &location.proxy_to;

    if root.is_some() {
        return handle_static(req, location).await;
    } else if proxy_to.is_some() {
        return handle_proxy(req, location).await;
    }

    return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(format!(
            "Error: No handle for location: {}",
            req.uri(),
        )))
        .unwrap();
}

pub async fn handle_server(req: Request<Body>, server: &ServerConfig) -> Response<Body> {
    let path = req.uri();

    for location in server.locations.as_ref().unwrap() {
        if path.path().starts_with(&location.path) {
            return handle_location(req, location).await;
        }
    }

    return Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(format!(
            "Error: Location Not Found, Path: {}",
            req.uri(),
        )))
        .unwrap();
}

pub async fn handle_request(
    State(mut worker): State<Worker>,
    req: Request<Body>,
) -> Response<Body> {
    // Host getter.
    let host_header = req.headers().get("host");
    let host: &str;

    if host_header.is_some() {
        host = host_header.unwrap().to_str().unwrap();
    } else {
        host = "";
    }

    // Server getter.
    let mut server = worker.get_site(host.to_string());

    if server.is_none() {
        server = worker.get_default_site();
    }

    if server.is_some() {
        handle_server(req, server.unwrap()).await
    } else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!(
                "Error: Server Not Found, Host: {}, Path: {}",
                host,
                req.uri(),
            )))
            .unwrap();
    }
}
