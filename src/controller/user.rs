use actix_web::{
    HttpResponse,
    web,
};
use super::super::service::user::{service_user_index, service_user_show};
use super::super::repository::user::RepositoryUser;
use super::super::response::{
    user_index::response_user_index,
    user_show::response_user_show,
};
use super::super::request::user::{RequestUserIndex, RequestUserShow};

pub fn user_index(payload: web::Query<RequestUserIndex>) -> HttpResponse {
    let repository_user = RepositoryUser::new();
    let domain_users = &service_user_index(
        repository_user,
        payload.page,
        payload.page_size,
    );
    response_user_index(domain_users)
}

pub fn user_show(payload: web::Query<RequestUserShow>) -> HttpResponse {
    let repository_user = RepositoryUser::new();
    let domain_user = &service_user_show(
        repository_user,
        payload.user_id,
    );
    response_user_show(domain_user)
}