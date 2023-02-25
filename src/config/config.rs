use serde::{Deserialize, Serialize};

use super::server_config::ServerConfig;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub include: Option<Vec<String>>,
    #[serde(rename = "server")]
    pub servers: Option<Vec<ServerConfig>>,
}
