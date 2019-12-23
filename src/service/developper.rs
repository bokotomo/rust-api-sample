use super::super::{
    domain,
    repository,
};

pub fn index(page: i32, page_size: i32) -> (Vec<domain::developper::DomainDevelopper>, i32) {
    let repository_develop = repository::developper::RepositoryDevelopper::new();
    let total = repository_develop.find_developper_total(page_size);
    let domain_developpers = repository_develop.find_developpers(page, page_size);
    (domain_developpers, total)
}