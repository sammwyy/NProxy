use std::{path::PathBuf, str::FromStr};

use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse};

use crate::config::server_config::ServerLocationConfig;

pub fn try_server_file(
    req: &HttpRequest,
    root: &PathBuf,
    resource: &String,
) -> Option<HttpResponse> {
    if resource.contains("..") {
        return Some(HttpResponse::BadRequest().body(format!("Invalid path: {}", resource)));
    }

    let resource = resource.split("?").next().unwrap_or("");
    let file_path = &root.join(resource.replacen("/", "", 1));
    let file = NamedFile::open(file_path);

    if file.is_ok() {
        return Some(file.unwrap().into_response(&req));
    } else {
        return None;
    }
}

pub async fn handle_static(req: HttpRequest, location: &ServerLocationConfig) -> HttpResponse {
    let path = location.root.as_ref().unwrap();
    let root: PathBuf = PathBuf::from_str(path.as_str()).unwrap();
    let resource: String;
    let index = &location.index;
    let fallback = &location.fallback;
    let rewrite = &location.rewrite;

    if rewrite.is_some() {
        let rewrite = rewrite.as_ref().unwrap().as_str();
        resource = rewrite.replace("{path}", req.path());
    } else {
        resource = req.path().to_string();
    }

    let mut response = try_server_file(&req, &root, &resource);

    if response.is_none() && index.is_some() {
        let index = index.as_ref().unwrap();
        let index_resource = format!("{}/{}", resource, index);
        response = try_server_file(&req, &root, &index_resource);
    }

    if response.is_none() && fallback.is_some() {
        let fallback = fallback.as_ref().unwrap();
        response = try_server_file(&req, &root, &fallback);
    }

    if response.is_some() {
        return response.unwrap();
    } else {
        return HttpResponse::NotFound().body(format!("Resource not found: {}", resource));
    }
}
