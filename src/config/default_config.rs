use super::{
    config::Config,
    server_config::{ServerConfig, ServerLocationConfig},
};

pub fn default_config() -> Config {
    Config {
        include: Some(["./sites".to_string()].to_vec()),
        servers: Some(
            [ServerConfig {
                listen: Some("0.0.0.0:80".to_string()),
                server_name: Some("*".to_string()),
                locations: Some(
                    [ServerLocationConfig {
                        path: "/".to_string(),
                        root: Some("/var/www/html".to_string()),
                        ..Default::default()
                    }]
                    .to_vec(),
                ),
                ..Default::default()
            }]
            .to_vec(),
        ),
    }
}
