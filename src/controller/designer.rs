use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    request,
    response,
};

pub fn index(payload: web::Query<request::designer::Index>) -> HttpResponse {
    // ok1
    let (domain_designer, total) = &service::designer::index(
        11,//payload.page,
        payload.page_size,
    );
    response::designer_index::response(domain_designer, &total)
}