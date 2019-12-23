use actix_web::{
    HttpResponse,
};
use super::super::{
    service,
    response,
    repository
};

pub fn develop_index() -> HttpResponse {
    let repository_develop = repository::develop::RepositoryDevelop::new();
    let (domain_develop_trends, domain_develop_popularities) = &service::develop::index(
        repository_develop,
    );
    response::develop_index::response(domain_develop_trends, domain_develop_popularities)
}