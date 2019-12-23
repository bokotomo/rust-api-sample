use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    response,
    request,
};

pub fn user_index(payload: web::Query<request::user::Index>) -> HttpResponse {
    let domain_users = &service::user::index(
        payload.page,
        payload.page_size,
    );
    response::user_index::response(domain_users)
}

pub fn user_show(payload: web::Query<request::user::Show>) -> HttpResponse {
    let domain_user = &service::user::show(payload.user_id);
    response::user_show::response(domain_user)
}