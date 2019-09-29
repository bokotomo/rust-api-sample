use actix_web::{
    HttpResponse,
    web,
};
use super::super::service::job::{service_job_index, service_job_show};
use super::super::repository::job::RepositoryJob;
use super::super::response::job::{response_job_index, response_job_show};
use super::super::request::job::{RequestJobIndex, RequestJobShow};

pub fn job_index(payload: web::Query<RequestJobIndex>) -> HttpResponse {
    let repository_job = RepositoryJob::new();
    let domain_jobs = &service_job_index(
        repository_job,
        payload.page,
        payload.page_size,
    );
    response_job_index(domain_jobs)
}

pub fn job_show(payload: web::Query<RequestJobShow>) -> HttpResponse {
    let repository_job = RepositoryJob::new();
    let domain_job = &service_job_show(
        repository_job,
        payload.job_id,
    );
    response_job_show(domain_job)
}