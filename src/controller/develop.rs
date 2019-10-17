use actix_web::{
    HttpResponse,
    web,
};
use super::super::service::develop::{
    service_developper_index,
    service_develop_index,
};
use super::super::request::developper::RequestDevelopperIndex;
use super::super::response::{
    developper_index::response_developper_index,
    develop_index::response_develop_index,
};
use super::super::repository::develop::RepositoryDevelop;

pub fn developper_index(payload: web::Query<RequestDevelopperIndex>) -> HttpResponse {
    let repository_develop = RepositoryDevelop::new();
    let (domain_developpers, total) = &service_developper_index(
        repository_develop,
        payload.page,
        payload.page_size,
    );
    response_developper_index(domain_developpers, &total)
}

pub fn develop_index() -> HttpResponse {
    let repository_develop = RepositoryDevelop::new();
    let (domain_develop_trends, domain_develop_popularities) = &service_develop_index(
        repository_develop,
    );
    response_develop_index(domain_develop_trends, domain_develop_popularities)
}