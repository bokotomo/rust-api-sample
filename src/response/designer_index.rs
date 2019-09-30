use actix_web::HttpResponse;
use super::super::domain::designer::DomainDesigner;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Designers {
    id: i32,
    good_total: i32,
    post_images: Vec<String>,
    user_id: i32,
    user_name: String,
    user_image: String,
    user_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DesignerIndex {
    total: i32,
    designers: Vec<Designers>,
}

pub fn response_designer_index(domain_designers: &Vec<DomainDesigner>, total: &i32) -> HttpResponse {
    let mut designers = Vec::with_capacity(domain_designers.len());

    for domain_designer in domain_designers {
        designers.push(Designers {
            id: *domain_designer.id(),
            post_images: domain_designer.post_images().to_vec(),
            good_total: *domain_designer.user_good_total(),
            user_id: *domain_designer.user_id(),
            user_name: domain_designer.user_name().to_string(),
            user_image: domain_designer.user_image().to_string(),
            user_location: domain_designer.user_location().to_string(),
        });
    }

    HttpResponse::Ok().json(DesignerIndex {
        total: *total,
        designers,
    })
}
