use actix_web::{
    HttpResponse,
    web,
};
use super::super::repository::design::{find_designs};
use super::super::response::design::{response_design_index};
use super::super::request::design::{RequestDesignIndex};

pub fn design_index(payload: web::Json<RequestDesignIndex>) -> HttpResponse {
    let domain_designs = &find_designs(payload.page, payload.page_size);
    let json_designs = response_design_index(domain_designs);

    HttpResponse::Ok().json(json_designs)
}