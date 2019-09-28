use actix_web::HttpResponse;
use super::super::domain::design::DomainDesign;
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
    pickups: Vec<Designs>,
    designs: Vec<Designs>,
}

pub fn response_design_index(domain_designs: &Vec<DomainDesign>, total: &i32) -> HttpResponse {
    let mut designs = Vec::new();
    let mut pickups = Vec::new();

    for domain_design in domain_designs {
        designs.push(Designs {
            id: *domain_design.id(),
            thumbnail: domain_design.thumbnail().to_string(),
            title: domain_design.title().to_string(),
            good: *domain_design.good(),
            comment: *domain_design.comment(),
            user_id: *domain_design.user_id(),
            user_name: domain_design.user_name().to_string(),
            user_image: domain_design.user_image().to_string(),
        });
    }
    for domain_design in domain_designs {
        pickups.push(Designs {
            id: *domain_design.id(),
            thumbnail: domain_design.thumbnail().to_string(),
            title: domain_design.title().to_string(),
            good: *domain_design.good(),
            comment: *domain_design.comment(),
            user_id: *domain_design.user_id(),
            user_name: domain_design.user_name().to_string(),
            user_image: domain_design.user_image().to_string(),
        });
    }
    let response_designs = DesignIndex {
        total: *total,
        designs,
        pickups,
    };

    HttpResponse::Ok().json(response_designs)
}