use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Show{
    pub job_id: i32,
}