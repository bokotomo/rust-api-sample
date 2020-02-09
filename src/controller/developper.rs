use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    request,
    response,
};

pub fn index(payload: web::Query<request::developper::Index>) -> HttpResponse {
    let (domain_developpers, total) = &service::developper::index(
        payload.page,
        payload.page_size,
    );
    response::developper_index::response(domain_developpers, &total)
}
