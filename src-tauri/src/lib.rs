// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use std::collections::HashMap;
use crate::action::init::global_init;
use crate::action::operate_es::{create_connection, get_indexs_by_config, test_connect};
use crate::action::search_es::{get_column_info, get_docment, get_docment_by_condition};
use crate::action::util::{error_response, get_connection_config, handle_res, success_response};
use crate::model::response::CommonResponse;
use crate::model::connect_config_model::ConnectionConfig;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

lazy_static::lazy_static! {
    static ref GLOBAL_CACHE :Mutex<HashMap<String, ConnectionConfig>> = Mutex::new(HashMap::new());
}

#[tauri::command]
fn test_connect_config(data: &str) -> String {
    println!("Hello, {}!", data);
    let obj1: ConnectionConfig = serde_json::from_str(data).unwrap();
    println!("前端传来的数据：{:?}", obj1);

    let da = test_connect(&obj1);

    match da {
        Ok(_data) => {
            println!("成功调用");
            serde_json::to_string(&CommonResponse {
                code: "200".to_string(),
                message: "".to_string(),
                data: None,
            })
            .unwrap()
        }
        Err(e) => {
            println!(
                "失败的结果编码：{},成功的消息:{}",
                e.get_code(),
                e.get_message()
            );
            serde_json::to_string(&CommonResponse {
                code: "500".to_string(),
                message: e.get_message(),
                data: None,
            })
            .unwrap()
        }
    }
}

#[tauri::command]
fn create_new_connection(data: &str) -> String {
    let mut obj1: ConnectionConfig = serde_json::from_str(data).unwrap();
    let da = create_connection(&mut obj1);

    match da {
        Ok(_) => {
            println!("成功调用");
            serde_json::to_string(&CommonResponse {
                code: "200".to_string(),
                message: "".to_string(),
                data: None,
            })
            .unwrap()
        }
        Err(e) => {
            println!("失败的结果编码：{},成功的消息:{}", e, e);
            serde_json::to_string(&CommonResponse {
                code: "500".to_string(),
                message: e,
                data: None,
            })
            .unwrap()
        }
    }
}

#[tauri::command]
fn get_all_connections() -> String {
    if let Ok(da) = get_connection_config() {
        let res = serde_json::to_string(&CommonResponse {
            code: "200".to_string(),
            message: "".to_string(),
            data: Some(serde_json::to_value(da).unwrap()),
        })
        .unwrap();
        res
    } else {
        serde_json::to_string(&CommonResponse {
            code: "500".to_string(),
            message: "get connection failed".to_string(),
            data: None,
        })
        .unwrap()
    }
}

#[tauri::command]
fn get_all_indexs(conn_key: &str) -> String {
    let cache = GLOBAL_CACHE.lock().unwrap();
    match cache.get(conn_key) {
        Some(conn_config) => {
            let rst = get_indexs_by_config(conn_config);
            match rst {
                Ok(da) => success_response(da),
                Err(e) => error_response("500", &e.to_string()),
            }
        }
        None => error_response("500", "key is error"),
    }
}

#[tauri::command]
fn get_columns_by_index(conn_key: &str, index_name: &str) ->String {
    let cache = GLOBAL_CACHE.lock().unwrap();

    match cache.get(conn_key) {
        Some(conn_config) => {
            let rst: Result<Vec<model::es_column::Column>, String> = get_column_info(conn_config,index_name);
            match rst {
                Ok(da) => success_response(da),
                Err(e) => error_response("500", &e),
            }
        }
        None => error_response("500", "key is error"),
    }
}

#[tauri::command]
fn get_all_docment(conn_key: &str, index_name: &str) ->String{
    let cache = GLOBAL_CACHE.lock().unwrap();

    match cache.get(conn_key) {
        Some(conn_config) => {
            let rst = get_docment(conn_config,index_name);
            handle_res(rst)
        }
        None => error_response("500", "key is error"),
    }
}

#[tauri::command]
fn query_document_by_query_param(conn_key: &str, index_name: &str,condition:&str) -> String{
    let cache = GLOBAL_CACHE.lock().unwrap();
    match cache.get(conn_key) {
        Some(connect_config) =>{
            handle_res(get_docment_by_condition(connect_config, index_name,condition))
        },
        None => error_response("500", "key is error"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    global_init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            test_connect_config,
            create_new_connection,
            get_all_connections,
            get_all_indexs,
            get_columns_by_index,
            get_all_docment,
            query_document_by_query_param
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod model;
pub mod action;