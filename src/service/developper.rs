use super::super::{
    domain,
    repository,
};

pub fn index(repository_develop: repository::develop::RepositoryDevelop, page: i32, page_size: i32) -> (Vec<domain::developper::DomainDevelopper>, i32) {
    let total = repository_develop.find_developper_total(page_size);
    let domain_developpers = repository_develop.find_developpers(page, page_size);
    (domain_developpers, total)
}