use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{io, path};
use crate::model::response::CommonResponse;

use crate::action::operate_es::get_app_path;
use crate::model::connect_config_model::ConnectionConfig;

pub fn get_connection_config() -> Result<Vec<ConnectionConfig>, String> {
    let root_path = get_app_path().map_err(|e| {
        println!("Error: {}", e);
        return format!("Failed to get app path,reason:{}", e);
    })?;

    let connect_config_file_path = format!(
        "{}{}connect_config",
        root_path,
        path::MAIN_SEPARATOR.to_string()
    );
    if fs::metadata(connect_config_file_path.clone()).is_err() {
        // 检查文件是否存在
        let _rst = fs::File::create(connect_config_file_path.clone());
    }
    match read_lines(connect_config_file_path) {
        Ok(lines) => {
            let mut data = Vec::new();

            for line in lines {
                let line = line.map_err(|e| {
                    println!("Error: {}", e);
                    return format!("Failed to read line");
                })?;

                if let Ok(connect_config) = serde_json::from_str(line.trim()) {
                    data.push(connect_config);
                } else {
                    println!("serialzed failed,{}",line);
                    continue;
                }
            }

            Ok(data)
        }
        Err(e) => Err(format!("Failed to read lines: {}", e)),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}


// 辅助函数：构建成功响应
pub fn success_response(data: impl serde::Serialize) -> String {
    serde_json::to_string(&CommonResponse {
        code: "200".to_string(),
        message: "".to_string(),
        data: Some(serde_json::to_value(data).unwrap()),
    })
    .unwrap()
}

// 辅助函数：构建错误响应
pub fn error_response(code: &str, message: &str) -> String {
    serde_json::to_string(&CommonResponse {
        code: code.to_string(),
        message: message.to_string(),
        data: None,
    })
    .unwrap()
}


pub fn handle_res(result:Result<impl serde::Serialize,String>) -> String{
    match result {
        Ok(data) => success_response(data),
        Err(e) => error_response("500", &e),
    }
}