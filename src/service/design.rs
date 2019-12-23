use super::super::{
    domain,
    repository,
};

pub fn index(page: i32, page_size: i32) -> (Vec<domain::design::DomainDesign>, i32) {
    let repository_design = repository::design::RepositoryDesign::new();
    let total = repository_design.find_designs_total(page_size);
    let domain_designs = repository_design.find_designs(page, page_size);
    (domain_designs, total)
}