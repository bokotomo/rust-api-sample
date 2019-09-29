use super::super::domain::developper::DomainDevelopper;
use super::super::repository::develop::RepositoryDevelop;

pub fn service_developper_index(repository_develop: RepositoryDevelop, page: i32, page_size: i32) -> (Vec<DomainDevelopper>, i32) {
    let total = repository_develop.find_developper_total(page_size);
    let domain_developpers = repository_develop.find_developpers(page, page_size);
    (domain_developpers, total)
}