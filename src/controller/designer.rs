use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    request,
    response,
};

pub fn desinger_index(payload: web::Query<request::designer::Index>) -> HttpResponse {
    let (domain_designer, total) = &service::designer::index(
        payload.page,
        payload.page_size,
    );
    response::designer_index::response(domain_designer, &total)
}