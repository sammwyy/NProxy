use actix_web::{HttpRequest, HttpResponse};

use crate::config::server_config::ServerLocationConfig;

pub async fn handle_proxy(req: HttpRequest, location: &ServerLocationConfig) -> HttpResponse {
    let root = &location.root;

    if root.is_some() {}

    HttpResponse::Ok().body("pwoxy uwu")
}
