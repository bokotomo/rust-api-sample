use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    repository,
    response,
    request,
};

pub fn user_index(payload: web::Query<request::user::Index>) -> HttpResponse {
    let repository_user = repository::user::RepositoryUser::new();
    let domain_users = &service::user::index(
        repository_user,
        payload.page,
        payload.page_size,
    );
    response::user_index::response(domain_users)
}

pub fn user_show(payload: web::Query<request::user::Show>) -> HttpResponse {
    let repository_user = repository::user::RepositoryUser::new();
    let domain_user = &service::user::show(
        repository_user,
        payload.user_id,
    );
    response::user_show::response(domain_user)
}