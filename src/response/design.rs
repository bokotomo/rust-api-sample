use super::super::domain::design::{DomainDesign};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDesignIndex {
    pub id: i32,
    pub thumbnail: String,
    pub title: String,
    pub good: i32,
    pub comment: i32,
    pub user_id: i32,
    pub user_name: String,
    pub user_image: String,
}

pub fn response_design_index(domain_designs: &Vec<DomainDesign>) -> Vec<ResponseDesignIndex> {
    let mut json_designs = Vec::new();
    
    for domain_design in domain_designs {
        json_designs.push(ResponseDesignIndex {
            id: domain_design.id,
            thumbnail: domain_design.thumbnail.to_string(),
            title: domain_design.title.to_string(),
            good: domain_design.good,
            comment: domain_design.comment,
            user_id: domain_design.user.id,
            user_name: domain_design.user.name.to_string(),
            user_image: domain_design.user.image.to_string(),
        });
    }

    json_designs
}