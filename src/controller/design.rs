use actix_web::{
    HttpResponse,
    web,
};
use super::super::service::design::{service_design_index, service_pickup_index, service_designer_index};
use super::super::request::design::{RequestDesignIndex, RequestDesignerIndex};
use super::super::response::design::{response_design_index, response_pickup_index, response_designer_index};
use super::super::repository::design::RepositoryDesign;

pub fn design_index(payload: web::Query<RequestDesignIndex>) -> HttpResponse {
    let repository_design = RepositoryDesign::new();
    let (domain_designs, total) = &service_design_index(
        repository_design,
        payload.page,
        payload.page_size,
    );
    response_design_index(domain_designs, &total)
}

pub fn pickup_index() -> HttpResponse {
    let repository_design = RepositoryDesign::new();
    let domain_designs = &service_pickup_index(
        repository_design,
    );
    response_pickup_index(domain_designs)
}

pub fn desinger_index(payload: web::Query<RequestDesignerIndex>) -> HttpResponse {
    let repository_design = RepositoryDesign::new();
    let (domain_designer, total) = &service_designer_index(
        repository_design,
        payload.page,
        payload.page_size,
    );
    response_designer_index(domain_designer, &total)
}