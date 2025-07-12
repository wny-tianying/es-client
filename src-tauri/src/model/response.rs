use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonResponse {
    pub(crate) code:String,
    pub(crate) message: String,
    pub(crate) data: Option<serde_json::Value>,
}

impl CommonResponse {
    pub fn new(code:String,message:String,data:Option<serde_json::Value>) -> CommonResponse {
        CommonResponse{code:code,message:message,data:data}
    }
}