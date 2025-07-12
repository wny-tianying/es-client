use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDataReq{
    // 分页大小
    pub size:i32,
    // 索引的位置
    pub index:i32,
    // 查询的json
    pub query_json:String,
    // 索引的名称
    pub index_name:String,
}