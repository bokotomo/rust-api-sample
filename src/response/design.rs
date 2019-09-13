use actix_web::{HttpResponse};
use super::super::domain::design::{DomainDesign};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Designs {
    id: i32,
    thumbnail: String,
    title: String,
    good: i32,
    comment: i32,
    user_id: i32,
    user_name: String,
    user_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DesignIndex {
    total: i32,
    designs: Designs,
}

pub fn response_design_index(domain_designs: &Vec<DomainDesign>, total: &i32) -> HttpResponse {
    let mut response_designs = Vec::new();
    for domain_design in domain_designs {
        response_designs.push(DesignIndex {
            total: *total,
            designs: Designs {
                id: domain_design.id(),
                thumbnail: domain_design.thumbnail(),
                title: domain_design.title(),
                good: domain_design.good(),
                comment: domain_design.comment(),
                user_id: domain_design.user_id(),
                user_name: domain_design.user_name(),
                user_image: domain_design.user_image(),
            },
        });
    }

    HttpResponse::Ok().json(response_designs)
}