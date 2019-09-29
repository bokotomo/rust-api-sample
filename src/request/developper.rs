use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDevelopperIndex {
    pub page: i32,
    pub page_size: i32,
}