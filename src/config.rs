use std::{fs, io::Error};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigTomlServer {
    pub cgrpc_token: Option<String>, // Administrator Token, used to invoke cgrpc reqs. If not preset will default to no protection.
    pub port: String,
}
impl Default for ConfigTomlServer {
    fn default() -> Self {
        Self {
            port: "[::1]:50051".into(),
            cgrpc_token: None,
        }
    }
}
pub struct Config {
    pub config_toml: ConfigTomlServer,
}
impl Config {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut content: String = "".to_owned();
        let result: Result<String, Error> = fs::read_to_string("config.toml");
        if result.is_ok() {
            content = result.unwrap();
        };
        let config_toml: ConfigTomlServer = toml::from_str(&content).unwrap_or_else(|err| {
            println!("Failed to parse config file.");
            println!("{:#?}", err);
            ConfigTomlServer::default()
        });
        Self { config_toml }
    }
}
