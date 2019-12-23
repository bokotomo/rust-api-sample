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

pub fn design_index(payload: web::Query<request::design::RequestDesignIndex>) -> HttpResponse {
    let (domain_designs, total) = &service::design::service_design_index(
        payload.page,
        payload.page_size,
    );
    response::design_index::response(domain_designs, &total)
}

pub fn pickup_index() -> HttpResponse {
    let repository_design = repository::design::RepositoryDesign::new();
    let domain_designs = &service::design::service_pickup_index(
        repository_design,
    );
    response::pickup_index::response(domain_designs)
}

pub fn desinger_index(payload: web::Query<request::designer::RequestDesignerIndex>) -> HttpResponse {
    let (domain_designer, total) = &service::design::service_designer_index(
        payload.page,
        payload.page_size,
    );
    response::designer_index::response(domain_designer, &total)
}