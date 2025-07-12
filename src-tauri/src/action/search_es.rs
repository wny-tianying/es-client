use crate::action::init::get_run_time;
use crate::model::es_column::{Column, Document, EsResult};
use crate::{action::init::get_client, model::connect_config_model::ConnectionConfig};
use elasticsearch::indices::IndicesGetMappingParts;
use elasticsearch::SearchParts;

use serde_json::{from_value, json,  Value};
pub fn get_column_info(connect_config: &ConnectionConfig, index_name: &str) -> Result<Vec<Column>, String> {
    let client = get_client(connect_config);

    let runtime = get_run_time();
    runtime.block_on(async {
        let response = client
            .indices()
            .get_mapping(IndicesGetMappingParts::Index(&[index_name]))
            .send()
            .await.map_err(|e| {
                e.to_string().clone()
            })?;
        let body:Value = response.json().await.map_err(|e|  {e.to_string()})?;
        // 5. 提取所有字段名称（递归遍历映射）
    if let Some(mappings) = body[index_name]["mappings"].as_object() {
        let fields = extract_field_names(mappings);
        Ok(fields)
    } else {
        Err("没有字段".to_string())
    }
    })
}

// 获取文档
pub fn get_docment(connect_config: &ConnectionConfig, index_name: &str) -> Result<EsResult,String>{
    let runtime = get_run_time();
    runtime.block_on(async {
        let client = get_client(connect_config);
        let response = client
        .search(SearchParts::Index(&[index_name])) // 指定索引
        .body(json!({
            "query": {
                "match_all": {}
            },
            "size": 20  // 限制返回数量
        }))
        .send()
        .await.map_err(|e|{e.to_string()})?;

        let body:Value = response.json().await.map_err(|e|{
            println!("response json err");
            e.to_string()
        })?;

        // todo 下面的unwarp需要处理
        let hits = body["hits"]["hits"].as_array().unwrap();

        let total_count = body["hits"]["total"]["value"].as_i64().unwrap();

        let mut rest = vec![];
        for hint in hits {
            let doc:Document = from_value(hint.clone()).map_err(|e|{
                println!("json parse error:{:#?}",e.to_string());
                e.to_string()
            })?;
            rest.push(doc);
        }
        Ok(EsResult{
            index:0,
            total:total_count,
            documents:rest
        })
    })
    
}

// 获取文档
pub fn get_docment_by_condition(connect_config: &ConnectionConfig, index_name: &str,condition:&str) -> Result<EsResult,String>{
    let runtime = get_run_time();

    
    //  前端传过来的字符串带有\,所以去掉转换
    let query_es_json:Value = serde_json::from_str(&condition.replace(r"\", "")).unwrap();
    
    runtime.block_on(async {
        let client = get_client(connect_config);
        let response = client
        .search(SearchParts::Index(&[index_name])) // 指定索引
        .body(query_es_json)
        .send()
        .await.map_err(|e|{e.to_string()})?;

        let body:Value = response.json().await.map_err(|e|{
            println!("response json err");
            e.to_string()
        })?;

        println!("错误：{:#?}",body);

        // todo 下面的unwarp需要处理
        let hits = body["hits"]["hits"].as_array().unwrap();

        let total_count = body["hits"]["total"]["value"].as_i64().unwrap();

        let mut rest = vec![];
        for hint in hits {
            let doc:Document = from_value(hint.clone()).map_err(|e|{
                println!("json parse error:{:#?}",e.to_string());
                e.to_string()
            })?;
            rest.push(doc);
        }
        Ok(EsResult{
            index:0,
            total:total_count,
            documents:rest
        })
    })
    
}

fn extract_field_names(mapping: &serde_json::Map<String, Value>) -> Vec<Column> {
    let mut fields = Vec::new();
    let react_data = mapping["properties"].as_object().unwrap();
    for (field_name, field_def) in react_data {
        if let Some(properties) = field_def["properties"].as_object() {
            // 嵌套字段：递归处理
            let nested_fields = extract_field_names(properties);
            for nested in nested_fields {
                let column_name = format!("{}.{}", field_name, nested.column_name);
                fields.push(Column {
                    column_name: column_name,
                    column_type: nested.column_type,
                });
            }
        } else if field_def["type"].is_string() {
            println!("field_name:{}",field_name);
            // 普通字段
            fields.push(Column {
                column_name: field_name.clone(),
                column_type: field_def["type"].to_string().trim_matches('"').to_string(),
            });
        }
    }
    fields
}
