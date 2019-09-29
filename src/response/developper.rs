use actix_web::HttpResponse;
use super::super::domain::developper::DomainDevelopper;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Developper {
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
struct DevelopperIndex {
    total: i32,
    developpers: Vec<Developper>,
}

pub fn response_developper_index(domain_developpers: &Vec<DomainDevelopper>, total: &i32) -> HttpResponse {
    let mut developpers = Vec::with_capacity(domain_developpers.len());
    for domain_developper in domain_developpers {
        developpers.push(Developper {
            id: *domain_developper.id(),
            post_images: domain_developper.post_images().to_vec(),
            good_total: *domain_developper.user_good_total(),
            user_id: *domain_developper.user_id(),
            user_name: domain_developper.user_name().to_string(),
            user_image: domain_developper.user_image().to_string(),
            user_location: domain_developper.user_location().to_string(),
        });
    }
    let response_developpers = DevelopperIndex {
        total: *total,
        developpers,
    };

    HttpResponse::Ok().json(response_developpers)
}
