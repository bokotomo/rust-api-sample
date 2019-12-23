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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JobIndex {
    jobs: Vec<Job>,
}

pub fn response(domain_jobs: &Vec<DomainJob>) -> HttpResponse {
    let mut jobs = Vec::with_capacity(domain_jobs.len());
    for domain_job in domain_jobs {
        jobs.push(Job {
            id: *domain_job.id(),
            company_name: domain_job.company_name().to_string(),
            company_logo: domain_job.company_logo().to_string(),
            company_thumbnail: domain_job.company_thumbnail().to_string(),
            title: domain_job.title().to_string(),
            title_sub: domain_job.title_sub().to_string(),
            tag: domain_job.tag().to_string(),
        });
    }

    HttpResponse::Ok().json(JobIndex {
        jobs,
    })
}
