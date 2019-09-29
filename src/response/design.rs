use actix_web::HttpResponse;
use super::super::domain::{
    design::DomainDesign,
    designer::DomainDesigner,
};
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
struct Designers {
    id: i32,
    good_total: i32,
    post_images: String,
    user_id: i32,
    user_name: String,
    user_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DesignIndex {
    total: i32,
    designs: Vec<Designs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PickupIndex {
    pickups: Vec<Designs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DesignerIndex {
    total: i32,
    designers: Vec<Designers>,
}

pub fn response_design_index(domain_designs: &Vec<DomainDesign>, total: &i32) -> HttpResponse {
    let mut designs = Vec::new();

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
    let response_designs = DesignIndex {
        total: *total,
        designs,
    };

    HttpResponse::Ok().json(response_designs)
}

pub fn response_pickup_index(domain_designs: &Vec<DomainDesign>) -> HttpResponse {
    let mut pickups = Vec::new();
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
    let response_designs = PickupIndex {
        pickups,
    };

    HttpResponse::Ok().json(response_designs)
}

pub fn response_designer_index(domain_designers: &Vec<DomainDesigner>, total: &i32) -> HttpResponse {
    let mut designers = Vec::new();

    for domain_designer in domain_designers {
        designers.push(Designers {
            id: *domain_designer.id(),
            post_images: domain_designer.post_images().to_string(),
            good_total: *domain_designer.good_total(),
            user_id: *domain_designer.user_id(),
            user_name: domain_designer.user_name().to_string(),
            user_image: domain_designer.user_image().to_string(),
        });
    }
    let response_designers = DesignerIndex {
        total: *total,
        designers,
    };

    HttpResponse::Ok().json(response_designers)
}
