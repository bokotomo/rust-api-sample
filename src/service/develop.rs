use super::super::domain::{
    developper::DomainDevelopper,
    develop::DomainDevelop,
};
use super::super::repository::develop::RepositoryDevelop;

pub fn service_developper_index(repository_develop: RepositoryDevelop, page: i32, page_size: i32) -> (Vec<DomainDevelopper>, i32) {
    let total = repository_develop.find_developper_total(page_size);
    let domain_developpers = repository_develop.find_developpers(page, page_size);
    (domain_developpers, total)
}

pub fn service_develop_index(repository_develop: RepositoryDevelop) -> (Vec<DomainDevelop>, Vec<DomainDevelop>) {
    let domain_develop_trends = repository_develop.find_trends();
    let domain_develop_popularities = repository_develop.find_popularities();
    (domain_develop_trends, domain_develop_popularities)
}