use serde::{Deserialize, Serialize};
use crate::model::connect_config_model::ConnectionConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreConnectConfig {
    connection_config: ConnectionConfig,
    key:String,
}

impl StoreConnectConfig {
    pub fn new(connection_config: ConnectionConfig,key:String) -> Self {
        StoreConnectConfig{
            connection_config,
            key
        }
    }
}