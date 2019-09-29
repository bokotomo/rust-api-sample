use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDesignerIndex {
    pub page: i32,
    pub page_size: i32,
}