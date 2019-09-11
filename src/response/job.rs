use super::super::domain::job::{DomainJob};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseJobIndex {
    pub id: i32,
    pub company_name: String,
    pub company_logo: String,
    pub company_thumbnail: String,
    pub title: String,
    pub title_sub: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseJobShow {
    pub id: i32,
    pub company_name: String,
    pub company_logo: String,
    pub company_thumbnail: String,
    pub title: String,
    pub title_sub: String,
    pub tag: String,
    pub description: String,
    pub recruiment_period: String,
    pub location: String,
    pub salary: String,
    pub type_of_work: String,
    pub employees: i32,
}

pub fn response_job_index(domain_jobs: &Vec<DomainJob>) -> Vec<ResponseJobIndex> {
    let mut json_jobs = Vec::new();
    for domain_job in domain_jobs {
        json_jobs.push(ResponseJobIndex {
            id: domain_job.id,
            company_name: domain_job.company.name.to_string(),
            company_logo: domain_job.company.logo.to_string(),
            company_thumbnail: domain_job.company.thumbnail.to_string(),
            title: domain_job.title.to_string(),
            title_sub: domain_job.title_sub.to_string(),
            tag: domain_job.tag.to_string(),
        });
    }

    json_jobs
}

pub fn response_job_show(domain_job: &DomainJob) -> ResponseJobShow {
    ResponseJobShow {
        id: domain_job.id,
        company_name: domain_job.company.name.to_string(),
        company_logo: domain_job.company.logo.to_string(),
        company_thumbnail: domain_job.company.thumbnail.to_string(),
        title: domain_job.title.to_string(),
        title_sub: domain_job.title_sub.to_string(),
        tag: domain_job.tag.to_string(),
        description: domain_job.description.to_string(),
        recruiment_period: domain_job.recruiment_period.to_string(),
        location: domain_job.location.to_string(),
        salary: domain_job.salary.to_string(),
        type_of_work: domain_job.type_of_work.to_string(),
        employees: domain_job.employees,
    }
}