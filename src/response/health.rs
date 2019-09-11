use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseHealth {
    pub message: String,
}

pub fn response_health() -> ResponseHealth {
    ResponseHealth {
        message: "Success!".to_string(),
    }
}