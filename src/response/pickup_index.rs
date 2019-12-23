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
struct PickupIndex {
    pickups: Vec<Designs>,
}

pub fn response(domain_designs: &Vec<DomainDesign>) -> HttpResponse {
    let mut pickups = Vec::with_capacity(domain_designs.len());
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

    HttpResponse::Ok().json(PickupIndex {
        pickups,
    })
}