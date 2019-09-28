use actix_web::HttpResponse;
use super::super::domain::user::DomainUser;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserIndex {
    id: i32,
    title: String,
    good: i32,
    comment: i32,
}

pub fn response_user_index(domain_users: &Vec<DomainUser>) -> HttpResponse {
    let mut response_users = Vec::new();
    for domain_user in domain_users {
        response_users.push(UserIndex {
            id: domain_user.id(),
            title: domain_user.name(),
            good: domain_user.id(),
            comment: domain_user.id(),
        });
    }

    HttpResponse::Ok().json(response_users)
}