use anyhow::bail;
use axum::{routing::any, Router};
use std::{collections::HashMap, net::SocketAddr};

use crate::{config::server_config::ServerConfig, handlers::http::handle_request};

#[derive(Clone)]
pub struct Worker {
    pub sites: HashMap<String, ServerConfig>,
    pub listener: String,
}

impl Worker {
    pub fn new(listener: &str) -> Worker {
        Worker {
            sites: HashMap::new(),
            listener: listener.to_string(),
        }
    }

    pub fn add_site(&mut self, site: ServerConfig) -> Result<(), anyhow::Error> {
        let name = site.server_name.clone().unwrap_or("*".to_string());
        let old = self.sites.insert(name.clone(), site);

        if old.is_some() {
            bail!(format!(
                "Already exist a server with name {} in this listener ({})",
                name, self.listener
            ));
        }

        Ok(())
    }

    pub fn get_site(&mut self, hostname: String) -> Option<&ServerConfig> {
        return self.sites.get(&hostname);
    }

    pub fn get_default_site(&mut self) -> Option<&ServerConfig> {
        return self.get_site("*".to_string());
    }

    pub async fn start(self) -> Result<(), hyper::Error> {
        tracing_subscriber::fmt::init();

        let app = Router::new()
            .fallback(any(handle_request))
            .with_state(self.clone());
        let addr: SocketAddr = self.listener.parse().unwrap();

        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
    }
}
