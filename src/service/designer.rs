use super::super::domain;
use super::super::repository;

pub fn index(page: i32, page_size: i32) -> (Vec<domain::designer::DomainDesigner>, i32) {
    let repository_design = repository::design::RepositoryDesign::new();
    let total = repository_design.find_designers_total(page_size);
    let domain_designers = repository_design.find_designers(page, page_size);
    (domain_designers, total)
}