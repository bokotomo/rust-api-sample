use super::super::domain::user::{DomainUser};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseUserIndex {
    pub id: i32,
    pub title: String,
    pub good: i32,
    pub comment: i32,
}

pub fn response_user_index(domain_users: &Vec<DomainUser>) -> Vec<ResponseUserIndex> {
    let mut json_users = Vec::new();
    for domain_user in domain_users {
        json_users.push(ResponseUserIndex {
            id: domain_user.id,
            title: domain_user.name.to_string(),
            good: domain_user.id,
            comment: domain_user.id,
        });
    }

    json_users
}