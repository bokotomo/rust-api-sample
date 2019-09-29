use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDesignIndex {
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDesignerIndex {
    pub page: i32,
    pub page_size: i32,
}