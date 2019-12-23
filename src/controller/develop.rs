use actix_web::{
    HttpResponse,
};
use super::super::{
    service,
    response,
};

pub fn develop_index() -> HttpResponse {
    let (domain_develop_trends, domain_develop_popularities) = &service::develop::index();
    response::develop_index::response(domain_develop_trends, domain_develop_popularities)
}