use actix_web::{
    HttpResponse,
    web,
};
use super::super::repository::job::{find_jobs, find_job};
use super::super::response::job::{response_job_index, response_job_show};
use super::super::request::job::{RequestJobIndex};

pub fn job_index(payload: web::Json<RequestJobIndex>) -> HttpResponse {
    let domain_jobs = &find_jobs(payload.page, payload.page_size);
    let json_jobs = response_job_index(domain_jobs);
    
    HttpResponse::Ok().json(json_jobs)
}

pub fn job_show(payload: web::Json<RequestJobIndex>) -> HttpResponse {
    let domain_job = &find_job(payload.page, payload.page_size);
    let json_job = response_job_show(domain_job);
    
    HttpResponse::Ok().json(json_job)
}