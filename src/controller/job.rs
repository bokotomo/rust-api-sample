use actix_web::{
    HttpResponse,
    web,
};
use super::super::{
    service,
    response,
    request,
};

pub fn index(payload: web::Query<request::job::Index>) -> HttpResponse {
    let domain_jobs = &service::job::index(
        payload.page,
        payload.page_size,
    );
    response::job_index::response(domain_jobs)
}

pub fn show(payload: web::Query<request::job::Show>) -> HttpResponse {
    let domain_job = &service::job::show(payload.job_id);
    response::job_show::response(domain_job)
}