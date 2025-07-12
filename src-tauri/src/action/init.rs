use elasticsearch::Elasticsearch;
use elasticsearch::auth::Credentials;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};

use crate::action::util::get_connection_config;
use crate::GLOBAL_CACHE;
use crate::model::connect_config_model::ConnectionConfig;
use tokio::runtime::{Builder, Runtime};

pub fn global_init(){
    init_global_variable();
}

fn init_global_variable(){
    let connection_configs = get_connection_config().unwrap();
    {
        let mut cache = GLOBAL_CACHE.lock().unwrap();
        for val in connection_configs {
            cache.insert(val.get_connect_name().to_string(), val);
        }
    }
    
}

// 获取客户端
pub fn get_client(connection_config: &ConnectionConfig) -> Elasticsearch{
    let url = format!(
        "{}://{}:{}/",
        connection_config.get_protocol(),
        connection_config.get_ip(),
        connection_config.get_port()
    );
    // 创建传输层
    // let transport = Transport::single_node(url.as_str()).unwrap();
    let credentials = Credentials::Basic(
        connection_config.get_username().to_string(),
        connection_config.get_password().to_string(),
    );
    let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url.parse().unwrap()))
        .auth(credentials)
        .build()
        .unwrap();

    // 创建客户端
    Elasticsearch::new(transport)
}

pub fn get_run_time() -> Runtime{
    Builder::new_current_thread().enable_all().build().unwrap()
}