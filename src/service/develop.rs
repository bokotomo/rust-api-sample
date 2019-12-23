use super::super::{
    domain,
    repository,
};

pub fn index() -> (Vec<domain::develop::DomainDevelop>, Vec<domain::develop::DomainDevelop>) {
    let repository_develop = repository::develop::RepositoryDevelop::new();
    let domain_develop_trends = repository_develop.find_trends();
    let domain_develop_popularities = repository_develop.find_popularities();
    (domain_develop_trends, domain_develop_popularities)
}