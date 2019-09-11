use actix_web::{
    HttpResponse,
    web,
};
use super::super::repository::user::{find_users};
use super::super::response::user::{response_user_index};
use super::super::request::user::{RequestUserIndex};

pub fn user_index(payload: web::Json<RequestUserIndex>) -> HttpResponse {
    let domain_users = &find_users(payload.page, payload.page_size);
    let json_users = response_user_index(domain_users);
    
    HttpResponse::Ok().json(json_users)
}