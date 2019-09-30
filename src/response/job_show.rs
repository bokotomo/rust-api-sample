use actix_web::HttpResponse;
use super::super::domain::job::DomainJob;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Job {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JobShow {
    job: Job,
}

pub fn response_job_show(domain_job: &DomainJob) -> HttpResponse {
    let job = Job {
        id: *domain_job.id(),
        company_name: domain_job.company_name().to_string(),
        company_logo: domain_job.company_logo().to_string(),
        company_thumbnail: domain_job.company_thumbnail().to_string(),
        title: domain_job.title().to_string(),
        title_sub: domain_job.title_sub().to_string(),
        tag: domain_job.tag().to_string(),
        description: domain_job.description().to_string(),
        recruiment_period: domain_job.recruitment_period().to_string(),
        location: domain_job.location().to_string(),
        salary: domain_job.salary().to_string(),
        type_of_work: domain_job.type_of_work().to_string(),
        employees: *domain_job.employees(),
    };

    HttpResponse::Ok().json(JobShow {
        job,
    })
}