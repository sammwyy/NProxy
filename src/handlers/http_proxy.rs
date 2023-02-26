use axum::response::Response;
use hyper::{Body, Request, StatusCode};

use crate::config::server_config::ServerLocationConfig;

pub async fn handle_proxy(_req: Request<Body>, _location: &ServerLocationConfig) -> Response<Body> {
    return Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(Body::from("Not implemented yet"))
        .unwrap();
}
