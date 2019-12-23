use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub number: i32,
}