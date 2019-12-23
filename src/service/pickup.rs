use super::super::{
    domain,
    repository,
};

pub fn index(repository_design: repository::design::RepositoryDesign) -> Vec<domain::design::DomainDesign> {
    let domain_designs = repository_design.find_pickups();
    domain_designs
}