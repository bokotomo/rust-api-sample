use actix_web::HttpResponse;
use super::super::domain::job::DomainJob;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JobIndex {
    id: i32,
    company_name: String,
    company_logo: String,
    company_thumbnail: String,
    title: String,
    title_sub: String,
    tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JobShow {
    id: i32,
    company_name: String,
    company_logo: String,
    company_thumbnail: String,
    title: String,
    title_sub: String,
    tag: String,
    description: String,
    recruiment_period: String,
    location: String,
    salary: String,
    type_of_work: String,
    employees: i32,
}

pub fn response_job_index(domain_jobs: &Vec<DomainJob>) -> HttpResponse {
    let mut response_jobs = Vec::new();
    for domain_job in domain_jobs {
        response_jobs.push(JobIndex {
            id: domain_job.id(),
            company_name: domain_job.company_name(),
            company_logo: domain_job.company_logo(),
            company_thumbnail: domain_job.company_thumbnail(),
            title: domain_job.title(),
            title_sub: domain_job.title_sub(),
            tag: domain_job.tag(),
        });
    }

    HttpResponse::Ok().json(response_jobs)
}

pub fn response_job_show(domain_job: &DomainJob) -> HttpResponse {
    let response_job = JobShow {
        id: domain_job.id(),
        company_name: domain_job.company_name(),
        company_logo: domain_job.company_logo(),
        company_thumbnail: domain_job.company_thumbnail(),
        title: domain_job.title(),
        title_sub: domain_job.title_sub(),
        tag: domain_job.tag(),
        description: domain_job.description(),
        recruiment_period: domain_job.recruitment_period(),
        location: domain_job.location(),
        salary: domain_job.salary(),
        type_of_work: domain_job.type_of_work(),
        employees: domain_job.employees(),
    };

    HttpResponse::Ok().json(response_job)
}