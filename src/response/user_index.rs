use actix_web::HttpResponse;
use super::super::domain::user::DomainUser;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserIndex {
    users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: i32,
    title: String,
    good: i32,
    comment: i32,
}

pub fn response_user_index(domain_users: &Vec<DomainUser>) -> HttpResponse {
    let mut users = Vec::with_capacity(domain_users.len());
    for domain_user in domain_users {
        users.push(User {
            id: *domain_user.id(),
            title: domain_user.name().to_string(),
            good: *domain_user.id(),
            comment: *domain_user.id(),
        });
    }

    HttpResponse::Ok().json(UserIndex {
        users,
    })
}
