use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    repository,
    response,
    request,
};

pub fn job_index(payload: web::Query<request::job::Index>) -> HttpResponse {
    let repository_job = repository::job::RepositoryJob::new();
    let domain_jobs = &service::job::index(
        repository_job,
        payload.page,
        payload.page_size,
    );
    response::job_index::response(domain_jobs)
}

pub fn job_show(payload: web::Query<request::job::Show>) -> HttpResponse {
    let repository_job = repository::job::RepositoryJob::new();
    let domain_job = &service::job::show(
        repository_job,
        payload.job_id,
    );
    response::job_show::response(domain_job)
}