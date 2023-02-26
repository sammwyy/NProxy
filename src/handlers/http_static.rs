use std::{path::PathBuf, str::FromStr};

use axum::response::Response;
use hyper::{header, Body, Request, StatusCode};
use tokio_util::io::ReaderStream;

use crate::{config::server_config::ServerLocationConfig, utils::mimetype::get_mime_by_filename};

pub async fn try_server_file(root: &PathBuf, resource: &String) -> Option<Response<Body>> {
    if resource.contains("..") {
        return Some(
            Response::builder()
                .status(StatusCode::NOT_IMPLEMENTED)
                .body(Body::from(format!("Invalid path: {}", resource)))
                .unwrap(),
        );
    }

    let resource = resource.split("?").next().unwrap_or("");
    let file_path = &root.join(resource.replacen("/", "", 1));
    let file = match tokio::fs::File::open(file_path).await {
        Ok(file) => file,
        Err(_err) => return None,
    };

    let stream = ReaderStream::new(file);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            get_mime_by_filename(file_path).unwrap_or("".to_string()),
        )
        .body(Body::wrap_stream(stream))
        .unwrap();

    return Some(response);
}

pub async fn handle_static(req: Request<Body>, location: &ServerLocationConfig) -> Response<Body> {
    let path = location.root.as_ref().unwrap();
    let root: PathBuf = PathBuf::from_str(path.as_str()).unwrap();
    let resource: String;
    let index = &location.index;
    let fallback = &location.fallback;
    let rewrite = &location.rewrite;

    if rewrite.is_some() {
        let rewrite = rewrite.as_ref().unwrap().as_str();
        resource = rewrite.replace("{path}", req.uri().path());
    } else {
        resource = req.uri().path().to_string();
    }

    let mut response = try_server_file(&root, &resource).await;

    if response.is_none() && index.is_some() {
        let index = index.as_ref().unwrap();
        let index_resource = format!("{}/{}", resource, index);
        response = try_server_file(&root, &index_resource).await;
    }

    if response.is_none() && fallback.is_some() {
        let fallback = fallback.as_ref().unwrap();
        response = try_server_file(&root, &fallback).await;
    }

    if response.is_some() {
        return response.unwrap();
    } else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("Resource not found: {}", resource,)))
            .unwrap();
    }
}
