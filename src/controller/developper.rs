use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    request,
    response,
    repository,
};

pub fn developper_index(payload: web::Query<request::developper::Index>) -> HttpResponse {
    let repository_develop = repository::develop::RepositoryDevelop::new();
    let (domain_developpers, total) = &service::developper::index(
        repository_develop,
        payload.page,
        payload.page_size,
    );
    response::developper_index::response(domain_developpers, &total)
}
