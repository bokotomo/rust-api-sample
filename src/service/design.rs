use super::super::domain::{
    design::DomainDesign,
    designer::DomainDesigner,
};
use super::super::repository::design::RepositoryDesign;

pub fn service_design_index(page: i32, page_size: i32) -> (Vec<DomainDesign>, i32) {
    let repository_design = RepositoryDesign::new();
    let total = repository_design.find_designs_total(page_size);
    let domain_designs = repository_design.find_designs(page, page_size);
    (domain_designs, total)
}

pub fn service_pickup_index(repository_design: RepositoryDesign) -> Vec<DomainDesign> {
    let domain_designs = repository_design.find_pickups();
    domain_designs
}

pub fn service_designer_index(page: i32, page_size: i32) -> (Vec<DomainDesigner>, i32) {
    let repository_design = RepositoryDesign::new();
    let total = repository_design.find_designers_total(page_size);
    let domain_designers = repository_design.find_designers(page, page_size);
    (domain_designers, total)
}