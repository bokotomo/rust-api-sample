use actix_web::{
    HttpResponse,
    web,
};
use super::super::service::design::{service_design_index};
use super::super::request::design::{RequestDesignIndex};
use super::super::response::design::{response_design_index};
use super::super::repository::design::{RepositoryDesign};

pub fn design_index(payload: web::Json<RequestDesignIndex>) -> HttpResponse {
    let repository_design = RepositoryDesign::new();
    let (domain_designs, total) = &service_design_index(
        repository_design,
        payload.page,
        payload.page_size,
    );
    response_design_index(domain_designs, total)
}