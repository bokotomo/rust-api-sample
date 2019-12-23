use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    request,
    response,
    repository
};

pub fn design_index(payload: web::Query<request::design::Index>) -> HttpResponse {
    let (domain_designs, total) = &service::design::index(
        payload.page,
        payload.page_size,
    );
    response::design_index::response(domain_designs, &total)
}

pub fn pickup_index() -> HttpResponse {
    let repository_design = repository::design::RepositoryDesign::new();
    let domain_designs = &service::pickup::index(
        repository_design,
    );
    response::pickup_index::response(domain_designs)
}
