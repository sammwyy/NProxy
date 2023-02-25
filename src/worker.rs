use actix_web::{middleware, web, App, HttpServer, Result};
use anyhow::bail;
use std::{collections::HashMap, sync::Mutex};

use crate::{config::server_config::ServerConfig, handler::handle_request};

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

    pub async fn start(&self) -> std::io::Result<()> {
        let data = web::Data::new(Mutex::new(self.clone()));

        let server = HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .wrap(middleware::NormalizePath::default())
                .route("{path:.*}", web::route().to(handle_request))
        });
        server.bind(self.listener.to_string())?.run().await
    }
}
