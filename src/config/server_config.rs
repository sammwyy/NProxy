use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerLocationConfig {
    pub path: String,

    // Static files
    pub root: Option<String>,
    pub fallback: Option<String>,
    pub rewrite: Option<String>,

    // Proxy
    pub proxy_to: Option<String>,
    pub ip_forward: Option<bool>,
    pub host_forward: Option<bool>,
    pub trust_proxy: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    // HTTP
    pub listen: Option<String>,
    pub server_name: Option<String>,

    // SSL
    pub listen_ssl: Option<String>,
    pub ssl_certificate: Option<String>,
    pub ssl_certificate_key: Option<String>,

    // Components
    #[serde(rename = "location")]
    pub locations: Option<Vec<ServerLocationConfig>>,
}
