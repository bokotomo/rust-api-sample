use super::super::domain::job::{DomainJob};
use super::super::repository::job::{RepositoryJob};

pub fn service_job_index(repository_job: RepositoryJob, page: i32, page_size: i32) -> Vec<DomainJob> {
    let domain_jobs = repository_job.find_jobs(page, page_size);
    domain_jobs
}

pub fn service_job_show(repository_job: RepositoryJob, page: i32, page_size: i32) -> DomainJob {
    let domain_job = repository_job.find_job(page, page_size);
    domain_job
}