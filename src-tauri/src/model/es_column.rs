use serde::{Serialize,Deserialize};
use serde_json::Map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Column{
    pub column_name:String,
    pub column_type:String,
}

impl Column{
    fn new(column_name:String,column_type:String) -> Column{
        Column{
            column_name,column_type
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Document{
    #[serde(rename = "_id")]
    pub id:String,
    #[serde(rename = "_source")]
    pub source: Map<String,serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EsResult{
    pub index:i32,
    pub total:i64,
    pub documents:Vec<Document>
}