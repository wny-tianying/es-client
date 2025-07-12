use std::collections::hash_map::Values;
use crate::model::connect_config_model::ConnectionConfig;
use crate::model::error::ConnectEsError;
use crate::action::init::get_client;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::{Elasticsearch};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path;
use tokio::runtime::{Builder, Runtime};
use crate::GLOBAL_CACHE;

// 用于测试连接是否能正常连接
pub fn test_connect(connection_config: &ConnectionConfig) -> Result<String, ConnectEsError> {
    // 创建客户端
    let client = get_client(connection_config);

    let rt = get_run_time();

    rt.block_on(async {
        match check_es_connection(&client).await {
            Ok(_) => Ok("200".to_string()),
            Err(d) => Err(d),
        }
    })
}

fn get_run_time() -> Runtime{
    Builder::new_current_thread().enable_all().build().unwrap()
}


pub fn create_connection(param: &mut ConnectionConfig) ->Result<String, String> {
    let rst  = test_connect(&param);

    if rst.is_ok() {
        println!("create_connection");
        // 1.将连接的配置保存在本地
        if let root_dir = get_app_path(){
            let connect_config_file_path = 
                format!("{}{}connect_config",root_dir?.to_string(),path::MAIN_SEPARATOR.to_string());
            println!("配置文件路径：{}",connect_config_file_path);

            let mut cache = GLOBAL_CACHE.lock().unwrap();

            if let Some(config) = cache.clone().get(param.get_connect_name()) {
                println!("data:{}",config.get_connect_name());
                cache.insert(param.get_connect_name().to_string(), param.clone());
                update_config_file(connect_config_file_path,cache.values());
            }else {
                println!("size:{}",cache.values().len());
                cache.insert(param.get_connect_name().to_string(), param.clone());
                append_config_file(connect_config_file_path,param.clone());
            }

            println!("operate file successfully created!");
            Ok("success".to_string())
        }else {
            Err("Connection failed.".to_string())
        }
        // 2.返回创建成功的标识
    }else { 
        return Err("Connection failed.".to_string());
    }
}

/**
 * 获取连接下的所有的索引
 */
pub fn get_indexs_by_config(connection_config: &ConnectionConfig) -> Result< Vec<serde_json::Value>, elasticsearch::Error>{
    let runtime =get_run_time();
    runtime.block_on(async{
        let client = get_client(connection_config);
        let reponse = client
        .cat().indices(CatIndicesParts::Index(&["*"]))
        .format("json")
        .send()
        .await?;
        let indexs : Vec<serde_json::Value> = reponse.json().await?;
        Ok(indexs)
    })
}

fn update_config_file(filepath:String, values: Values<String, ConnectionConfig>) ->Result<(), String> {
    println!("update_config_file");
    if let Ok(mut file) = OpenOptions::new().write(true).create(true)
        .append(false).open(&filepath){
        let mut contents = String::from("");
        for (val) in values {
            contents.push_str(&format!("{}\n",serde_json::to_string(val).unwrap()));
            println!("neirong:{}",val.get_connect_name());
        }
        writeln!(file, "{}", contents.to_string());
        file.flush();
        Ok(())
    }else {
        Err(format!("File {} not found.", filepath))
    }
}

fn append_config_file(filepath:String,connection_config: ConnectionConfig) ->Result<(), String> {
    println!("append_config_file");
    if let Ok(mut file) = OpenOptions::new().write(true).append(true).create(true).open(&filepath){
        let contents = serde_json::to_string(&connection_config).map_err(|e| {"Failed to serialize connection config object.".to_string()})?;
        writeln!(file, "{}", contents);
        file.flush();
        Ok(())
    }else {
        Err(format!("File {} not found.", filepath))
    }
}

pub async fn check_es_connection(client: &Elasticsearch) -> Result<(), ConnectEsError> {
    let response = client.ping().send().await;

    match response {
        Ok(res) => {
            if !res.status_code().is_success() {
                return Err(ConnectEsError::new(
                    format!("{}", res.status_code()),
                    "Elasticsearch ping failed with status: ".to_string(),
                ));
            }
            Ok(())
        }
        Err(d) => {
            println!("异常：{:?}", d);
            let mut err = d.source().unwrap();
            let root = find_root_error(err);
            Err(ConnectEsError::new("500".to_string(), root.to_string()))
        }
    }
}

fn find_root_error(mut err: &dyn Error) -> &dyn Error {
    while let Some(source) = err.source() {
        err = source;
    }
    err
}



///
/// 
pub fn get_app_path() -> Result<String,String>{
    std::env::current_exe()
        .map_err(|_e| format!("Failed to get current executable path"))
        .and_then(|path| {
            let mut exe_dir = path.clone();
            exe_dir.pop();
            exe_dir.to_str()
                .map(|s| s.to_string())
            .ok_or_else(|| format!("路径包含无效的unicode字符"))
        })
}


