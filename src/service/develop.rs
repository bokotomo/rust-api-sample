use super::super::domain;
use super::super::repository;

pub fn index(repository_develop: repository::develop::RepositoryDevelop) -> (Vec<domain::develop::DomainDevelop>, Vec<domain::develop::DomainDevelop>) {
    let domain_develop_trends = repository_develop.find_trends();
    let domain_develop_popularities = repository_develop.find_popularities();
    (domain_develop_trends, domain_develop_popularities)
}