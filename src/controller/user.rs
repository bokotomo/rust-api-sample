use actix_web::{
    HttpResponse,
    web,
};
use super::super::service::user::{service_user_index};
use super::super::repository::user::{RepositoryUser};
use super::super::response::user::{response_user_index};
use super::super::request::user::{RequestUserIndex};

pub fn user_index(payload: web::Query<RequestUserIndex>) -> HttpResponse {
    let repository_user = RepositoryUser::new();
    let domain_users = &service_user_index(
        repository_user,
        payload.page,
        payload.page_size,
    );
    response_user_index(domain_users)
}