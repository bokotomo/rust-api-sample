use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUserIndex {
    pub page: i32,
    pub page_size: i32,
}