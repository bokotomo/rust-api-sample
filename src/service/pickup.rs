use super::super::{
    domain,
    repository,
};

pub fn index() -> Vec<domain::design::DomainDesign> {
    let repository_design = repository::design::RepositoryDesign::new();
    let domain_designs = repository_design.find_pickups();
    domain_designs
}